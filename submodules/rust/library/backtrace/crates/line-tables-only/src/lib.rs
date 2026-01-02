mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use backtrace :: Backtrace ;}
mkuse!{use libc :: c_void ;}
mkuse!{use std :: path :: Path ;}
mkuse!{use std :: ptr :: addr_of_mut ;}
mkitem!{pub type Callback = extern "C" fn (data : * mut c_void) ;}
mkitem!{unsafe extern "C" { fn foo (cb : Callback , data : * mut c_void) ; }}

macro_rules! store_backtrace_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function store_backtrace in module {}", module_path!());
    };
}

mkfn!{
    store_backtrace_introspect!();
    extern "C" fn store_backtrace (data : * mut c_void) { let bt = backtrace :: Backtrace :: new () ; unsafe { * data . cast :: < Option < Backtrace > > () = Some (bt) } ; }
}

macro_rules! assert_contains_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assert_contains in module {}", module_path!());
    };
}

mkfn!{
    assert_contains_introspect!();
    fn assert_contains (backtrace : & Backtrace , expected_name : & str , expected_file : & str , expected_line : u32 ,) { let expected_file = Path :: new (expected_file) ; for frame in backtrace . frames () { for symbol in frame . symbols () { if let Some (name) = symbol . name () { if name . as_bytes () == expected_name . as_bytes () { assert ! (symbol . filename () . unwrap () . ends_with (expected_file)) ; assert_eq ! (symbol . lineno () , Some (expected_line)) ; return ; } } } } panic ! ("symbol {expected_name:?} not found in backtrace: {backtrace:?}") ; }
}

macro_rules! backtrace_works_with_line_tables_only_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function backtrace_works_with_line_tables_only in module {}", module_path!());
    };
}

mkfn!{
    backtrace_works_with_line_tables_only_introspect!();
    # [doc = " Verifies that when debug info includes only lines tables the generated"] # [doc = " backtrace is still generated successfully. The test exercises behaviour"] # [doc = " that failed previously when compiling with clang -g1."] # [doc = ""] # [doc = " The test case uses C rather than rust, since at that time when it was"] # [doc = " written the debug info generated at level 1 in rustc was essentially"] # [doc = " the same as at level 2."] # [test] # [cfg_attr (windows , ignore)] fn backtrace_works_with_line_tables_only () { let mut backtrace : Option < Backtrace > = None ; unsafe { foo (store_backtrace , addr_of_mut ! (backtrace) . cast :: < c_void > ()) } ; let backtrace = backtrace . expect ("backtrace") ; assert_contains (& backtrace , "foo" , "src/callback.c" , 13) ; assert_contains (& backtrace , "bar" , "src/callback.c" , 9) ; assert_contains (& backtrace , "baz" , "src/callback.c" , 5) ; }
} 
            }}