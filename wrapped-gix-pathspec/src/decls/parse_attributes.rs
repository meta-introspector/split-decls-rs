macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! parse_attributes {
    () => {
        deps!();
        fn parse_attributes (input : & [u8]) -> Result < Vec < gix_attributes :: Assignment > , Error > { if input . is_empty () { return Err (Error :: EmptyAttribute) ; } let unescaped = unescape_attribute_values (input . into ()) ? ; gix_attributes :: parse :: Iter :: new (unescaped . as_bstr ()) . map (| res | res . map (gix_attributes :: AssignmentRef :: to_owned)) . collect :: < Result < Vec < _ > , _ > > () . map_err (| e | Error :: InvalidAttribute { attribute : e . attribute }) }
    };
}

parse_attributes!()