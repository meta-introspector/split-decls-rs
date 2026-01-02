mkuse!{use split_decls_genesis :: ourprelude :: * ;}
mkuse!{use std :: io :: prelude :: * ;}
mkuse!{use std :: io ;}

macro_rules! demangle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function demangle in module {}", module_path!());
    };
}

mkfn!{
    demangle_introspect!();
    pub fn demangle (writer : & mut Write , s : & str) -> io :: Result < () > { let mut valid = true ; let mut inner = s ; if s . len () > 4 && s . starts_with ("_ZN") && s . ends_with ("E") { inner = & s [3 .. s . len () - 1] ; } else if s . len () > 3 && s . starts_with ("ZN") && s . ends_with ("E") { inner = & s [2 .. s . len () - 1] ; } else { valid = false ; } if valid { let mut chars = inner . chars () ; while valid { let mut i = 0 ; for c in chars . by_ref () { if c . is_numeric () { i = i * 10 + c as usize - '0' as usize ; } else { break } } if i == 0 { valid = chars . next () . is_none () ; break } else if chars . by_ref () . take (i - 1) . count () != i - 1 { valid = false ; } } } if ! valid { return writer . write_all (s . as_bytes ()) ; } let mut first = true ; while ! inner . is_empty () { if ! first { try ! (writer . write_all (b"::")) ; } else { first = false ; } let mut rest = inner ; while rest . chars () . next () . unwrap () . is_numeric () { rest = & rest [1 ..] ; } let i : usize = inner [.. (inner . len () - rest . len ())] . parse () . unwrap () ; inner = & rest [i ..] ; rest = & rest [.. i] ; while ! rest . is_empty () { if rest . starts_with ("$") { macro_rules ! demangle { ($ ($ pat : expr , => $ demangled : expr) ,*) => ({ $ (if rest . starts_with ($ pat) { try ! (writer . write_all ($ demangled)) ; rest = & rest [$ pat . len () ..] ; } else) * { try ! (writer . write_all (rest . as_bytes ())) ; break ; } }) } demangle ! { "$SP$" , => b"@" , "$BP$" , => b"*" , "$RF$" , => b"&" , "$LT$" , => b"<" , "$GT$" , => b">" , "$LP$" , => b"(" , "$RP$" , => b")" , "$C$" , => b"," , "$u7e$" , => b"~" , "$u20$" , => b" " , "$u27$" , => b"'" , "$u5b$" , => b"[" , "$u5d$" , => b"]" } } else { let idx = rest . find ('$') . unwrap_or (rest . len ()) ; try ! (writer . write_all (rest [.. idx] . as_bytes ())) ; rest = & rest [idx ..] ; } } } Ok (()) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkitem!{macro_rules ! t { ($ a : expr , $ b : expr) => ({ let mut m = Vec :: new () ; super :: demangle (& mut m , $ a) . unwrap () ; assert_eq ! (String :: from_utf8 (m) . unwrap () , $ b) ; }) }}

macro_rules! demangle_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function demangle in module {}", module_path!());
    };
}

mkfn!{
    demangle_introspect!();
    #[test] fn demangle () { t ! ("test" , "test") ; t ! ("_ZN4testE" , "test") ; t ! ("_ZN4test" , "_ZN4test") ; t ! ("_ZN4test1a2bcE" , "test::a::bc") ; }
}

macro_rules! demangle_dollars_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function demangle_dollars in module {}", module_path!());
    };
}

mkfn!{
    demangle_dollars_introspect!();
    #[test] fn demangle_dollars () { t ! ("_ZN4$RP$E" , ")") ; t ! ("_ZN8$RF$testE" , "&test") ; t ! ("_ZN8$BP$test4foobE" , "*test::foob") ; t ! ("_ZN9$u20$test4foobE" , " test::foob") ; }
}

macro_rules! demangle_many_dollars_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function demangle_many_dollars in module {}", module_path!());
    };
}

mkfn!{
    demangle_many_dollars_introspect!();
    #[test] fn demangle_many_dollars () { t ! ("_ZN13test$u20$test4foobE" , "test test::foob") ; t ! ("_ZN12test$BP$test4foobE" , "test*test::foob") ; }
}

macro_rules! demangle_windows_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function demangle_windows in module {}", module_path!());
    };
}

mkfn!{
    demangle_windows_introspect!();
    #[test] fn demangle_windows () { t ! ("ZN4testE" , "test") ; t ! ("ZN13test$u20$test4foobE" , "test test::foob") ; t ! ("ZN12test$RF$test4foobE" , "test&test::foob") ; }
} 
            }}