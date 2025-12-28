macro_rules! deps {
    () => {
        FieldInfo!();
    };
}

macro_rules! make_ule_fields {
    () => {
        deps!();
        # [doc = " Make corresponding ULE fields for each field"] pub (crate) fn make_ule_fields (fields : & [FieldInfo]) -> Vec < TokenStream2 > { fields . iter () . map (| f | { let ty = & f . field . ty ; let ty = quote ! (<# ty as zerovec :: ule :: AsULE >:: ULE) ; let setter = f . setter () ; let vis = & f . field . vis ; quote ! (# vis # setter # ty) }) . collect :: < Vec < _ > > () }
    };
}

make_ule_fields!()