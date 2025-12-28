macro_rules! deps {
    () => {
        NFA!();
        Remappable!();
        StateID!();
    };
}

macro_rules! impl_439 {
    () => {
        deps!();
        impl Remappable for noncontiguous :: NFA { fn state_len (& self) -> usize { noncontiguous :: NFA :: states (self) . len () } fn swap_states (& mut self , id1 : StateID , id2 : StateID) { noncontiguous :: NFA :: swap_states (self , id1 , id2) } fn remap (& mut self , map : impl Fn (StateID) -> StateID) { noncontiguous :: NFA :: remap (self , map) } }
    };
}

impl_439!()