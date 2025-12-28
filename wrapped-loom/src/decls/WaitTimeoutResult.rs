macro_rules! WaitTimeoutResult {
    () => {
        # [doc = " A type indicating whether a timed wait on a condition variable returned due"] # [doc = " to a time out or not."] # [derive (Debug)] pub struct WaitTimeoutResult (bool) ;
    };
}

WaitTimeoutResult!()