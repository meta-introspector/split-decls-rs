macro_rules! deps {
    () => {
        StateID!();
        TransitionTable!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        # [cfg (feature = "dfa-build")] impl < T : AsMut < [u32] > > TransitionTable < T > { # [doc = " Returns the table as a slice of state IDs."] fn table_mut (& mut self) -> & mut [StateID] { wire :: u32s_to_state_ids_mut (self . table . as_mut ()) } }
    };
}

impl_33!();