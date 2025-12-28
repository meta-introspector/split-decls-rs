macro_rules! TryReadyError {
    () => {
        # [doc = " An error returned from the [`try_ready`] method."] # [doc = ""] # [doc = " Failed because none of the channel operations were ready."] # [doc = ""] # [doc = " [`try_ready`]: super::Select::try_ready"] # [derive (PartialEq , Eq , Clone , Copy , Debug)] pub struct TryReadyError ;
    };
}

TryReadyError!();