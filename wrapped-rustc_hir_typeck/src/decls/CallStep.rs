macro_rules! deps {
    () => {
        MethodCallee!();
    };
}

macro_rules! CallStep {
    () => {
        deps!();
        # [derive (Debug)] enum CallStep < 'tcx > { Builtin (Ty < 'tcx >) , DeferredClosure (LocalDefId , ty :: FnSig < 'tcx >) , # [doc = " Call overloading when callee implements one of the Fn* traits."] Overloaded (MethodCallee < 'tcx >) , }
    };
}

CallStep!();