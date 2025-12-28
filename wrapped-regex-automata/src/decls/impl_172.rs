macro_rules! deps {
    () => {
        HalfMatch!();
        OverlappingState!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl OverlappingState { # [doc = " Create a new overlapping state that begins at the start state of any"] # [doc = " automaton."] pub fn start () -> OverlappingState { OverlappingState { mat : None , id : None , at : 0 , next_match_index : None , rev_eoi : false , } } # [doc = " Return the match result of the most recent search to execute with this"] # [doc = " state."] # [doc = ""] # [doc = " A searches will clear this result automatically, such that if no"] # [doc = " match is found, this will correctly report `None`."] pub fn get_match (& self) -> Option < HalfMatch > { self . mat } }
    };
}

impl_172!()