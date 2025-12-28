macro_rules! deps {
    () => {
        RefSpecRef!();
        Needle!();
        Matcher!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < 'a > From < RefSpecRef < 'a > > for Matcher < 'a > { fn from (v : RefSpecRef < 'a >) -> Self { let mut m = Matcher { lhs : v . src . map (Into :: into) , rhs : v . dst . map (Into :: into) , } ; if m . rhs . is_none () { if let Some (src) = v . src { if must_use_pattern_matching (src) { m . lhs = Some (Needle :: Pattern (src)) ; } } } m } }
    };
}

impl_49!()