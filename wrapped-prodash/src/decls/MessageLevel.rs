macro_rules! MessageLevel {
    () => {
        # [doc = " The severity of a message"] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd)] pub enum MessageLevel { # [doc = " Rarely sent information related to the progress, not to be confused with the progress itself"] Info , # [doc = " Used to indicate that a task has failed, along with the reason"] Failure , # [doc = " Indicates a task was completed successfully"] Success , }
    };
}

MessageLevel!()