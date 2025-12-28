macro_rules! ReadyTimeoutError {
    () => {
        # [doc = " An error returned from the [`ready_timeout`] method."] # [doc = ""] # [doc = " Failed because none of the channel operations became ready before the timeout."] # [doc = ""] # [doc = " [`ready_timeout`]: super::Select::ready_timeout"] # [derive (PartialEq , Eq , Clone , Copy , Debug)] pub struct ReadyTimeoutError ;
    };
}

ReadyTimeoutError!()