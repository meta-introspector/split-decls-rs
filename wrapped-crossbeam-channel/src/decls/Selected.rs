macro_rules! deps {
    () => {
        Operation!();
    };
}

macro_rules! Selected {
    () => {
        deps!();
        # [doc = " Current state of a select or a blocking operation."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Selected { # [doc = " Still waiting for an operation."] Waiting , # [doc = " The attempt to block the current thread has been aborted."] Aborted , # [doc = " An operation became ready because a channel is disconnected."] Disconnected , # [doc = " An operation became ready because a message can be sent or received."] Operation (Operation) , }
    };
}

Selected!();