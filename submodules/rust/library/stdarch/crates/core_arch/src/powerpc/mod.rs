mkmod!{macros, { 
                getname!(macros);
                getsrc!(macros);
                getpath!(macros);
                get_deps!(macros);
                get_crates!(macros);
                mkinclude!(macros);
                 
            }}
mkmod!{altivec, { 
                getname!(altivec);
                getsrc!(altivec);
                getpath!(altivec);
                get_deps!(altivec);
                get_crates!(altivec);
                mkinclude!(altivec);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub use self :: altivec :: * ;}
mkmod!{vsx, { 
                getname!(vsx);
                getsrc!(vsx);
                getpath!(vsx);
                get_deps!(vsx);
                get_crates!(vsx);
                mkinclude!(vsx);
                 
            }}
mkuse!{# [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub use self :: vsx :: * ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! trap_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function trap in module {}", module_path!());
    };
}

mkfn!{
    trap_introspect!();
    # [doc = " Generates the trap instruction `TRAP`"] # [cfg_attr (test , assert_instr (trap))] # [inline] # [unstable (feature = "stdarch_powerpc" , issue = "111145")] pub unsafe fn trap () -> ! { crate :: intrinsics :: abort () }
}