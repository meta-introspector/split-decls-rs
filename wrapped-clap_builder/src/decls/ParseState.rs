macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! ParseState {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq)] pub (crate) enum ParseState { ValuesDone , Opt (Id) , Pos (Id) , }
    };
}

ParseState!()