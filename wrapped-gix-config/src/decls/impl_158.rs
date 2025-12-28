macro_rules! deps {
    () => {
        Header!();
        Error!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < 'a > Header < 'a > { # [doc = " Instantiate a new header either with a section `name`, e.g. \"core\" serializing to `[\"core\"]`"] # [doc = " or `[remote \"origin\"]` for `subsection` being \"origin\" and `name` being \"remote\"."] pub fn new (name : impl Into < Cow < 'a , str > > , subsection : impl Into < Option < Cow < 'a , BStr > > > ,) -> Result < Header < 'a > , Error > { let name = Name (validated_name (into_cow_bstr (name . into ())) ?) ; if let Some (subsection_name) = subsection . into () { Ok (Header { name , separator : Some (Cow :: Borrowed (" " . into ())) , subsection_name : Some (validated_subsection (subsection_name) ?) , }) } else { Ok (Header { name , separator : None , subsection_name : None , }) } } }
    };
}

impl_158!();