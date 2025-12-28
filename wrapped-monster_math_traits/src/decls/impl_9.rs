macro_rules! deps {
    () => {
        MonsterConformityChecker!();
        DummyMonsterConformityChecker!();
        Declaration!();
        BottPeriodicityTrait!();
        MonsterConstants!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl MonsterConformityChecker for DummyMonsterConformityChecker { fn check_conformity (& self , declaration : & Declaration , bott_periodicity_checker : Option < & dyn BottPeriodicityTrait > , constants : & dyn MonsterConstants ,) -> bool { let base_conformity = declaration . monster_factors . as_ref () . map_or (false , | factors | { ! factors . is_empty () && declaration . semantic_hash . is_some () }) ; if base_conformity { if let Some (bott_checker) = bott_periodicity_checker { println ! ("DummyMonsterConformityChecker: Performing Bott Periodicity check using period '{}'" , bott_checker . get_period ()) ; bott_checker . test_fixed_point_convergence () ; println ! ("DummyMonsterConformityChecker: Monster representation dimension: {}" , constants . get_representation_dimension ()) ; return bott_checker . monster_element (constants) > 0 ; } true } else { false } } }
    };
}

impl_9!()