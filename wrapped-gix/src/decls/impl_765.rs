macro_rules! deps {
    () => {
        Section!();
        Any!();
        Validate!();
    };
}

macro_rules! impl_765 {
    () => {
        deps!();
        impl < T : Validate > gix_config :: AsKey for Any < T > { fn as_key (& self) -> gix_config :: KeyRef < '_ > { self . try_as_key () . expect ("infallible") } fn try_as_key (& self) -> Option < KeyRef < '_ > > { let section_name = self . section . parent () . map_or_else (| | self . section . name () , Section :: name) ; let subsection_name = if self . section . parent () . is_some () { Some (self . section . name () . into ()) } else { None } ; let value_name = self . name ; gix_config :: KeyRef { section_name , subsection_name , value_name , } . into () } }
    };
}

impl_765!()