macro_rules! MixedProcMacroCrate {
    () => {
        # [derive (Diagnostic)] # [diag (interface_mixed_proc_macro_crate)] pub struct MixedProcMacroCrate ;
    };
}

MixedProcMacroCrate!();