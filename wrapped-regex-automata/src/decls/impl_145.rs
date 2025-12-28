macro_rules! deps {
    () => {
        NE!();
        StateMut!();
        StateID!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        # [cfg (feature = "dfa-build")] impl < 'a > StateMut < 'a > { # [doc = " Sets the ith transition to the given state."] fn set_next_at (& mut self , i : usize , next : StateID) { let start = i * StateID :: SIZE ; let end = start + StateID :: SIZE ; wire :: write_state_id :: < wire :: NE > (next , & mut self . next [start .. end]) ; } }
    };
}

impl_145!();