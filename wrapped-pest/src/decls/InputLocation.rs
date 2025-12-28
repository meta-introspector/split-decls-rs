macro_rules! deps {
    () => {
        Span!();
        Error!();
    };
}

macro_rules! InputLocation {
    () => {
        deps!();
        # [doc = " Where an `Error` has occurred."] # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub enum InputLocation { # [doc = " `Error` was created by `Error::new_from_pos`"] Pos (usize) , # [doc = " `Error` was created by `Error::new_from_span`"] Span ((usize , usize)) , }
    };
}

InputLocation!()