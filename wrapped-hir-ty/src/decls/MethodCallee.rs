macro_rules! deps {
    () => {
        FnSig!();
    };
}

macro_rules! MethodCallee {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug)] pub (crate) struct MethodCallee < 'db > { # [doc = " Impl method ID, for inherent methods, or trait method ID, otherwise."] pub def_id : FunctionId , pub args : GenericArgs < 'db > , # [doc = " Instantiated method signature, i.e., it has been"] # [doc = " instantiated, normalized, and has had late-bound"] # [doc = " lifetimes replaced with inference variables."] pub sig : FnSig < 'db > , }
    };
}

MethodCallee!()