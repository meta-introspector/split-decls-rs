macro_rules! deps {
    () => {
        Match!();
        Transition!();
        State!();
        Look!();
        StateID!();
    };
}

macro_rules! impl_459 {
    () => {
        deps!();
        impl State { # [doc = " If this state is an unconditional epsilon transition, then this returns"] # [doc = " the target of the transition."] fn goto (& self) -> Option < StateID > { match * self { State :: Empty { next } => Some (next) , State :: Union { ref alternates } if alternates . len () == 1 => { Some (alternates [0]) } State :: UnionReverse { ref alternates } if alternates . len () == 1 => { Some (alternates [0]) } _ => None , } } # [doc = " Returns the heap memory usage, in bytes, of this state."] fn memory_usage (& self) -> usize { match * self { State :: Empty { .. } | State :: ByteRange { .. } | State :: Look { .. } | State :: CaptureStart { .. } | State :: CaptureEnd { .. } | State :: Fail | State :: Match { .. } => 0 , State :: Sparse { ref transitions } => { transitions . len () * mem :: size_of :: < Transition > () } State :: Union { ref alternates } => { alternates . len () * mem :: size_of :: < StateID > () } State :: UnionReverse { ref alternates } => { alternates . len () * mem :: size_of :: < StateID > () } } } }
    };
}

impl_459!()