macro_rules! deps {
    () => {
        Result!();
        RaFixtureAnalysis!();
        UpmapFromRaFixture!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl UpmapFromRaFixture for TextRange { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , virtual_file_id : FileId , _real_file_id : FileId ,) -> Result < Self , () > { analysis . map_range_up (virtual_file_id , self) . next () . ok_or (()) } }
    };
}

impl_125!()