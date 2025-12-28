macro_rules! SelectTimeoutError {
    () => {
        # [doc = " An error returned from the [`select_timeout`] method."] # [doc = ""] # [doc = " Failed because none of the channel operations became ready before the timeout."] # [doc = ""] # [doc = " [`select_timeout`]: super::Select::select_timeout"] # [derive (PartialEq , Eq , Clone , Copy , Debug)] pub struct SelectTimeoutError ;
    };
}

SelectTimeoutError!()