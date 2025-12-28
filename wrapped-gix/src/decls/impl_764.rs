macro_rules! deps {
    () => {
        Error!();
        Section!();
        Link!();
        Note!();
        Key!();
        SubSectionRequirement!();
        Validate!();
        Any!();
    };
}

macro_rules! impl_764 {
    () => {
        deps!();
        impl < T : Validate > Key for Any < T > { fn name (& self) -> & str { self . name } fn validate (& self , value : & BStr) -> Result < () , config :: tree :: key :: validate :: Error > { Ok (self . validate . validate (value) ?) } fn section (& self) -> & dyn Section { self . section } fn subsection_requirement (& self) -> Option < & SubSectionRequirement > { self . subsection_requirement . as_ref () } fn link (& self) -> Option < & Link > { self . link . as_ref () } fn note (& self) -> Option < & Note > { self . note . as_ref () } }
    };
}

impl_764!()