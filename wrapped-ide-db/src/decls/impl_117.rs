macro_rules! deps {
    () => {
        UpmapFromRaFixture!();
        RaFixtureAnalysis!();
        Result!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < T : UpmapFromRaFixture > UpmapFromRaFixture for Option < T > { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > { Ok (match self { Some (it) => Some (it . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id) ?) , None => None , }) } }
    };
}

impl_117!()