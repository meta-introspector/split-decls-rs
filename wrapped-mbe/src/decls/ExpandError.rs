macro_rules! deps {
    () => {
        ExpandErrorKind!();
    };
}

macro_rules! ExpandError {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Clone , Hash)] pub struct ExpandError { pub inner : Arc < (Span , ExpandErrorKind) > , }
    };
}

ExpandError!()