macro_rules! deps {
    () => {
        Anchored!();
        Start!();
        StartTable!();
        StateID!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        # [cfg (feature = "dfa-build")] impl < T : AsMut < [u32] > > StartTable < T > { # [doc = " Set the start state for the given index and pattern."] # [doc = ""] # [doc = " If the pattern ID or state ID are not valid, then this will panic."] fn set_start (& mut self , anchored : Anchored , start : Start , id : StateID) { let start_index = start . as_usize () ; let index = match anchored { Anchored :: No => start_index , Anchored :: Yes => self . stride + start_index , Anchored :: Pattern (pid) => { let pid = pid . as_usize () ; let len = self . pattern_len . expect ("start states for each pattern enabled") ; assert ! (pid < len , "invalid pattern ID {pid:?}") ; self . stride . checked_mul (pid) . unwrap () . checked_add (self . stride . checked_mul (2) . unwrap ()) . unwrap () . checked_add (start_index) . unwrap () } } ; self . table_mut () [index] = id ; } # [doc = " Returns the table as a mutable slice of state IDs."] fn table_mut (& mut self) -> & mut [StateID] { wire :: u32s_to_state_ids_mut (self . table . as_mut ()) } }
    };
}

impl_38!();