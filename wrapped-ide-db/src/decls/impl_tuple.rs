macro_rules! deps {
    () => {
        UpmapFromRaFixture!();
        Result!();
        RaFixtureAnalysis!();
    };
}

macro_rules! impl_tuple {
    () => {
        deps!();
        macro_rules ! impl_tuple { () => { } ; ($ first : ident , $ ($ rest : ident ,) *) => { impl < $ first : UpmapFromRaFixture , $ ($ rest : UpmapFromRaFixture ,) * > UpmapFromRaFixture for ($ first , $ ($ rest ,) *) { fn upmap_from_ra_fixture (self , analysis : & RaFixtureAnalysis , virtual_file_id : FileId , real_file_id : FileId ,) -> Result < Self , () > { # [allow (non_snake_case)] let ($ first , $ ($ rest ,) *) = self ; Ok (($ first . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id) ?, $ ($ rest . upmap_from_ra_fixture (analysis , virtual_file_id , real_file_id) ?,) *)) } } impl_tuple ! ($ ($ rest ,) *) ; } ; }
    };
}

impl_tuple!()