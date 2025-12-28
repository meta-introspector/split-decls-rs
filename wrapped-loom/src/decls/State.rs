macro_rules! State {
    () => {
        # [derive (Debug , Clone , Copy)] pub (crate) enum State { Runnable { unparked : bool } , Blocked (# [allow (dead_code)] Location) , Yield , Terminated , }
    };
}

State!();