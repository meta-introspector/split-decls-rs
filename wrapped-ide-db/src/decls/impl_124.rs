macro_rules! deps {
    () => {
        RaFixtureAnalysis!();
        Result!();
        UpmapFromRaFixture!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl UpmapFromRaFixture for TextSize { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , virtual_file_id : FileId , _real_file_id : FileId ,) -> Result < Self , () > { analysis . map_offset_up (virtual_file_id , self) . ok_or (()) } }
    };
}

impl_124!();