macro_rules! deps {
    () => {
        LintLevelQueryMap!();
        LintLevelsBuilder!();
    };
}

macro_rules! impl_341 {
    () => {
        deps!();
        impl < 'tcx > LintLevelsBuilder < '_ , LintLevelQueryMap < 'tcx > > { fn add_id (& mut self , hir_id : HirId) { self . provider . cur = hir_id ; self . add (self . provider . attrs . get (hir_id . local_id) , hir_id == hir :: CRATE_HIR_ID , Some (hir_id) ,) ; } }
    };
}

impl_341!();