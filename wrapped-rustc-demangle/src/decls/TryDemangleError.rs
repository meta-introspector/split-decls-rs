macro_rules! TryDemangleError {
    () => {
        # [doc = " Error returned from the `try_demangle` function below when demangling fails."] # [derive (Debug , Clone)] pub struct TryDemangleError { _priv : () , }
    };
}

TryDemangleError!();