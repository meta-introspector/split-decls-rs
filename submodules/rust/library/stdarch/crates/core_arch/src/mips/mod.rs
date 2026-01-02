mkmod!{msa, { 
                getname!(msa);
                getsrc!(msa);
                getpath!(msa);
                get_deps!(msa);
                get_crates!(msa);
                mkinclude!(msa);
                 
            }}
mkuse!{# [cfg (target_feature = "fp64")] # [unstable (feature = "stdarch_mips" , issue = "111198")] pub use self :: msa :: * ;}
mkuse!{# [cfg (test)] use stdarch_test :: assert_instr ;}

macro_rules! break__introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function break_ in module {}", module_path!());
    };
}

mkfn!{
    break__introspect!();
    # [doc = " Generates the trap instruction `BREAK`"] # [cfg_attr (test , assert_instr (break))] # [inline] # [unstable (feature = "stdarch_mips" , issue = "111198")] pub unsafe fn break_ () -> ! { crate :: intrinsics :: abort () }
}