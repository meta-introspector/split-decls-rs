macro_rules! WaitTimeoutResult {
    () => {
        # [doc = " A type indicating whether a timed wait on a condition variable returned"] # [doc = " due to a time out or not."] # [derive (Debug , PartialEq , Eq , Copy , Clone)] pub struct WaitTimeoutResult (bool) ;
    };
}

WaitTimeoutResult!();