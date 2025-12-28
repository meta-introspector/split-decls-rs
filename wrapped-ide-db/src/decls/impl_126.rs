macro_rules! deps {
    () => {
        UpmapFromRaFixture!();
        Result!();
        RaFixtureAnalysis!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl UpmapFromRaFixture for FilePositionWrapper < FileId > { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , _virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > { Ok (FilePositionWrapper { file_id : real_file_id , offset : self . offset . upmap_from_ra_fixture (analysis , self . file_id , real_file_id) ? , }) } }
    };
}

impl_126!();