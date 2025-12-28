macro_rules! deps {
    () => {
        MatchError!();
        StartKind!();
        Anchored!();
    };
}

macro_rules! enforce_anchored_consistency {
    () => {
        deps!();
        # [doc = " Returns an error if the start state configuration does not support the"] # [doc = " desired search configuration. See the internal 'AhoCorasick::start_kind'"] # [doc = " field docs for more details."] fn enforce_anchored_consistency (have : StartKind , want : Anchored ,) -> Result < () , MatchError > { match have { StartKind :: Both => Ok (()) , StartKind :: Unanchored if ! want . is_anchored () => Ok (()) , StartKind :: Unanchored => Err (MatchError :: invalid_input_anchored ()) , StartKind :: Anchored if want . is_anchored () => Ok (()) , StartKind :: Anchored => Err (MatchError :: invalid_input_unanchored ()) , } }
    };
}

enforce_anchored_consistency!();