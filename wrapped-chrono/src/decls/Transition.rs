macro_rules! deps {
    () => {
        NaiveDateTime!();
        FixedOffset!();
    };
}

macro_rules! Transition {
    () => {
        deps!();
        # [cfg (windows)] # [derive (Copy , Clone , Eq , PartialEq)] struct Transition { transition_utc : NaiveDateTime , offset_before : FixedOffset , offset_after : FixedOffset , }
    };
}

Transition!();