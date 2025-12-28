macro_rules! ReceiverValidityError {
    () => {
        # [doc = " Error cases which may be returned from `receiver_is_valid`. These error"] # [doc = " cases are generated in this function as they may be unearthed as we explore"] # [doc = " the `autoderef` chain, but they're converted to diagnostics in the caller."] enum ReceiverValidityError { # [doc = " The self type does not get to the receiver type by following the"] # [doc = " autoderef chain."] DoesNotDeref , # [doc = " A type was found which is a method type parameter, and that's not allowed."] MethodGenericParamUsed , }
    };
}

ReceiverValidityError!()