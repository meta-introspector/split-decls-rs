macro_rules! deps {
    () => {
        Result!();
        UpmapFromRaFixture!();
        RaFixtureAnalysis!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        # [allow (clippy :: disallowed_types)] impl < K : UpmapFromRaFixture + Hash + Eq , V : UpmapFromRaFixture , S : BuildHasher + Default > UpmapFromRaFixture for std :: collections :: HashMap < K , V , S > { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > { upmap_collection (self , analysis , virtual_file_id , real_file_id) } }
    };
}

impl_120!()