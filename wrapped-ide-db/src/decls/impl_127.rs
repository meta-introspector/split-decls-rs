macro_rules! deps {
    () => {
        UpmapFromRaFixture!();
        Result!();
        RaFixtureAnalysis!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl UpmapFromRaFixture for FileRangeWrapper < FileId > { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , _virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > { Ok (FileRangeWrapper { file_id : real_file_id , range : self . range . upmap_from_ra_fixture (analysis , self . file_id , real_file_id) ? , }) } }
    };
}

impl_127!();