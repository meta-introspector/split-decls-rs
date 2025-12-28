macro_rules! ClosureSignatures {
    () => {
        # [derive (Debug)] struct ClosureSignatures < 'tcx > { # [doc = " The signature users of the closure see."] bound_sig : ty :: PolyFnSig < 'tcx > , # [doc = " The signature within the function body."] # [doc = " This mostly differs in the sense that lifetimes are now early bound and any"] # [doc = " opaque types from the signature expectation are overridden in case there are"] # [doc = " explicit hidden types written by the user in the closure signature."] liberated_sig : ty :: FnSig < 'tcx > , }
    };
}

ClosureSignatures!();