macro_rules! deps {
    () => {
        Match!();
        OverlappingState!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl OverlappingState { # [doc = " Create a new overlapping state that begins at the start state."] pub fn start () -> OverlappingState { OverlappingState { mat : None , id : None , at : 0 , next_match_index : None } } # [doc = " Return the match result of the most recent search to execute with this"] # [doc = " state."] # [doc = ""] # [doc = " Every search will clear this result automatically, such that if no"] # [doc = " match is found, this will always correctly report `None`."] pub fn get_match (& self) -> Option < Match > { self . mat } }
    };
}

impl_36!()