macro_rules! LeapSecond {
    () => {
        # [doc = " Leap second of a TZif file"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] pub (super) struct LeapSecond { # [doc = " Unix leap time"] unix_leap_time : i64 , # [doc = " Leap second correction"] correction : i32 , }
    };
}

LeapSecond!();