macro_rules! deps {
    () => {
        DynCompatibilityViolation!();
        HirDatabase!();
    };
}

macro_rules! dyn_compatibility_with_callback {
    () => {
        deps!();
        pub fn dyn_compatibility_with_callback < F > (db : & dyn HirDatabase , trait_ : TraitId , cb : & mut F ,) -> ControlFlow < () > where F : FnMut (DynCompatibilityViolation) -> ControlFlow < () > , { let interner = DbInterner :: new_with (db , Some (trait_ . krate (db)) , None) ; for super_trait in elaborate :: supertrait_def_ids (interner , trait_ . into ()) . skip (1) { if db . dyn_compatibility_of_trait (super_trait . 0) . is_some () { cb (DynCompatibilityViolation :: HasNonCompatibleSuperTrait (trait_)) ? ; } } dyn_compatibility_of_trait_with_callback (db , trait_ , cb) }
    };
}

dyn_compatibility_with_callback!()