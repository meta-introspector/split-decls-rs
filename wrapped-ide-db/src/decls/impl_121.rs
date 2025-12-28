macro_rules! deps {
    () => {
        UpmapFromRaFixture!();
        RaFixtureAnalysis!();
        Result!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        # [allow (clippy :: disallowed_types)] impl < V : UpmapFromRaFixture , S : BuildHasher + Default > UpmapFromRaFixture for std :: collections :: HashMap < FileId , V , S > { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , _virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > { if self . is_empty () { return Ok (self) ; } let result = self . into_iter () . filter_map (| (virtual_file_id , value) | { Some ((real_file_id , value . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id) . ok () ? ,)) }) . collect :: < std :: collections :: HashMap < _ , _ , _ > > () ; if result . is_empty () { Err (()) } else { Ok (result) } } }
    };
}

impl_121!()