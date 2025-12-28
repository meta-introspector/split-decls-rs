macro_rules! CountError {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone , Hash)] pub enum CountError { OutOfBounds , Misplaced , }
    };
}

CountError!();