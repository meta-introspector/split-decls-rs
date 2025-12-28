macro_rules! deps {
    () => {
        UnparkToken!();
    };
}

macro_rules! ParkResult {
    () => {
        deps!();
        # [doc = " Result of a park operation."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub enum ParkResult { # [doc = " We were unparked by another thread with the given token."] Unparked (UnparkToken) , # [doc = " The validation callback returned false."] Invalid , # [doc = " The timeout expired."] TimedOut , }
    };
}

ParkResult!();