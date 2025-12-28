macro_rules! IncludingTupleField {
    () => {
        pub (super) struct IncludingTupleField (pub (super) bool) ;
    };
}

IncludingTupleField!()