macro_rules! deps {
    () => {
        InflateState!();
    };
}

macro_rules! ResetPolicy {
    () => {
        deps!();
        # [doc = " Tag that determines reset policy of [InflateState](struct.InflateState.html)"] pub trait ResetPolicy { # [doc = " Performs reset"] fn reset (& self , state : & mut InflateState) ; }
    };
}

ResetPolicy!();