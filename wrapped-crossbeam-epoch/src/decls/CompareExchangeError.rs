macro_rules! deps {
    () => {
        Pointer!();
        Pointable!();
        Shared!();
    };
}

macro_rules! CompareExchangeError {
    () => {
        deps!();
        # [doc = " The error returned on failed compare-and-swap operation."] pub struct CompareExchangeError < 'g , T : ? Sized + Pointable , P : Pointer < T > > { # [doc = " The value in the atomic pointer at the time of the failed operation."] pub current : Shared < 'g , T > , # [doc = " The new value, which the operation failed to store."] pub new : P , }
    };
}

CompareExchangeError!()