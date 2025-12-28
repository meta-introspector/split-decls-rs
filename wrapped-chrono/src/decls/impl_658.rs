macro_rules! deps {
    () => {
        NaiveDateTime!();
        FixedOffset!();
        Transition!();
    };
}

macro_rules! impl_658 {
    () => {
        deps!();
        # [cfg (windows)] impl Transition { fn new (transition_local : NaiveDateTime , offset_before : FixedOffset , offset_after : FixedOffset ,) -> Transition { let transition_utc = transition_local . overflowing_sub_offset (offset_before) ; Transition { transition_utc , offset_before , offset_after } } }
    };
}

impl_658!();