macro_rules! deps {
    () => {
        RaFixtureAnalysis!();
        UpmapFromRaFixture!();
        IsEmpty!();
        Result!();
    };
}

macro_rules! upmap_collection {
    () => {
        deps!();
        fn upmap_collection < T , Collection > (collection : Collection , analysis : & RaFixtureAnalysis , virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Collection , () > where T : UpmapFromRaFixture , Collection : IntoIterator < Item = T > + FromIterator < T > + IsEmpty , { if collection . is_empty () { return Ok (collection) ; } let result = collection . into_iter () . filter_map (| item | item . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id) . ok ()) . collect :: < Collection > () ; if result . is_empty () { Err (()) } else { Ok (result) } }
    };
}

upmap_collection!();