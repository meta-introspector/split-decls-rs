macro_rules! deps {
    () => {
        RaFixtureAnalysis!();
        Result!();
    };
}

macro_rules! UpmapFromRaFixture {
    () => {
        deps!();
        pub trait UpmapFromRaFixture : Sized { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > ; }
    };
}

UpmapFromRaFixture!();