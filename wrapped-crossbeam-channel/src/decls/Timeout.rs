macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! Timeout {
    () => {
        deps!();
        # [doc = " Determines when a select operation should time out."] # [derive (Clone , Copy , Eq , PartialEq)] enum Timeout { # [doc = " No blocking."] Now , # [doc = " Block forever."] Never , # [doc = " Time out after the time instant."] At (Instant) , }
    };
}

Timeout!()