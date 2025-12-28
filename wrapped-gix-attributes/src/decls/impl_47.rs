macro_rules! deps {
    () => {
        Outcome!();
        MatchLocation!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl MatchLocation { fn to_outer < 'a > (& self , out : & 'a Outcome) -> crate :: search :: MatchLocation < 'a > { crate :: search :: MatchLocation { source : self . source . and_then (| source | out . source_paths . resolve (source) . map (AsRef :: as_ref)) , sequence_number : self . sequence_number , } } }
    };
}

impl_47!();