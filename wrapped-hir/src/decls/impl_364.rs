macro_rules! deps {
    () => {
        Variance!();
        GenericParam!();
        ConstParam!();
        LifetimeParam!();
        GenericDef!();
        Module!();
        TypeParam!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        impl GenericParam { pub fn module (self , db : & dyn HirDatabase) -> Module { match self { GenericParam :: TypeParam (it) => it . module (db) , GenericParam :: ConstParam (it) => it . module (db) , GenericParam :: LifetimeParam (it) => it . module (db) , } } pub fn name (self , db : & dyn HirDatabase) -> Name { match self { GenericParam :: TypeParam (it) => it . name (db) , GenericParam :: ConstParam (it) => it . name (db) , GenericParam :: LifetimeParam (it) => it . name (db) , } } pub fn parent (self) -> GenericDef { match self { GenericParam :: TypeParam (it) => it . id . parent () . into () , GenericParam :: ConstParam (it) => it . id . parent () . into () , GenericParam :: LifetimeParam (it) => it . id . parent . into () , } } pub fn variance (self , db : & dyn HirDatabase) -> Option < Variance > { let parent = match self { GenericParam :: TypeParam (it) => it . id . parent () , GenericParam :: ConstParam (_) => return None , GenericParam :: LifetimeParam (it) => it . id . parent , } ; let generics = hir_ty :: generics :: generics (db , parent) ; let index = match self { GenericParam :: TypeParam (it) => generics . type_or_const_param_idx (it . id . into ()) ? , GenericParam :: ConstParam (_) => return None , GenericParam :: LifetimeParam (it) => generics . lifetime_idx (it . id) ? , } ; db . variances_of (parent) . get (index) . map (Into :: into) } }
    };
}

impl_364!();