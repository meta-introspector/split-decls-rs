macro_rules! UnparkReason {
    () => {
        # [doc = " An enum that reports whether a `Parker::park_timeout` or"] # [doc = " `Parker::park_deadline` returned because another thread called `unpark` or"] # [doc = " because of a timeout."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum UnparkReason { # [doc = " The park method returned due to a call to `unpark`."] Unparked , # [doc = " The park method returned due to a timeout."] Timeout , }
    };
}

UnparkReason!()