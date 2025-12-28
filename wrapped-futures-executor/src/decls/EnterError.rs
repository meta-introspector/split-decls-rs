macro_rules! EnterError {
    () => {
        # [doc = " An error returned by `enter` if an execution scope has already been"] # [doc = " entered."] pub struct EnterError { _priv : () , }
    };
}

EnterError!();