macro_rules! MethodCallee {
    () => {
        # [derive (Clone , Copy , Debug)] pub (crate) struct MethodCallee < 'tcx > { # [doc = " Impl method ID, for inherent methods, or trait method ID, otherwise."] pub def_id : DefId , pub args : GenericArgsRef < 'tcx > , # [doc = " Instantiated method signature, i.e., it has been"] # [doc = " instantiated, normalized, and has had late-bound"] # [doc = " lifetimes replaced with inference variables."] pub sig : ty :: FnSig < 'tcx > , }
    };
}

MethodCallee!();