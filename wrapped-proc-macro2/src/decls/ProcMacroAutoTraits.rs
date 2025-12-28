macro_rules! ProcMacroAutoTraits {
    () => {
        # [derive (Copy , Clone)] # [cfg_attr (all (procmacro2_semver_exempt , any (not (wrap_proc_macro) , super_unstable)) , derive (PartialEq , Eq))] pub (crate) struct ProcMacroAutoTraits (PhantomData < Rc < () > >) ;
    };
}

ProcMacroAutoTraits!();