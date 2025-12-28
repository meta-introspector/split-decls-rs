macro_rules! deps {
    () => {
        UpmapFromRaFixture!();
        Result!();
        RaFixtureAnalysis!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < T : UpmapFromRaFixture , const N : usize > UpmapFromRaFixture for SmallVec < [T ; N] > { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > { upmap_collection (self , analysis , virtual_file_id , real_file_id) } }
    };
}

impl_119!();