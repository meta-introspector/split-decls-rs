macro_rules! deps {
    () => {
        HirDatabase!();
        DynCompatibilityViolation!();
    };
}

macro_rules! dyn_compatibility_of_trait_with_callback {
    () => {
        deps!();
        pub fn dyn_compatibility_of_trait_with_callback < F > (db : & dyn HirDatabase , trait_ : TraitId , cb : & mut F ,) -> ControlFlow < () > where F : FnMut (DynCompatibilityViolation) -> ControlFlow < () > , { if generics_require_sized_self (db , trait_ . into ()) { cb (DynCompatibilityViolation :: SizedSelf) ? ; } if predicates_reference_self (db , trait_) { cb (DynCompatibilityViolation :: SelfReferential) ? ; } if bounds_reference_self (db , trait_) { cb (DynCompatibilityViolation :: SelfReferential) ? ; } let trait_data = trait_ . trait_items (db) ; for (_ , assoc_item) in & trait_data . items { dyn_compatibility_violation_for_assoc_item (db , trait_ , * assoc_item , cb) ? ; } ControlFlow :: Continue (()) }
    };
}

dyn_compatibility_of_trait_with_callback!();