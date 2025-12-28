macro_rules! deps {
    () => {
        RealRegexCaptures!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < 't > RegexCaptures for RealRegexCaptures < 't > { fn get (& self , i : usize) -> Option < & str > { self . captures . get (i) . map (| m | m . as_str ()) } fn len (& self) -> usize { self . captures . len () } }
    };
}

impl_3!()