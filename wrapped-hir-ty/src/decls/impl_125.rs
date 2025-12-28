macro_rules! deps {
    () => {
        HirDatabase!();
        EarlyBinder!();
        ImplTraits!();
        ImplTraitId!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < 'db > ImplTraitId < 'db > { # [inline] pub fn predicates (self , db : & 'db dyn HirDatabase) -> EarlyBinder < 'db , & 'db [Clause < 'db >] > { let (impl_traits , idx) = match self { ImplTraitId :: ReturnTypeImplTrait (owner , idx) => { (ImplTraits :: return_type_impl_traits (db , owner) , idx) } ImplTraitId :: TypeAliasImplTrait (owner , idx) => { (ImplTraits :: type_alias_impl_traits (db , owner) , idx) } } ; impl_traits . as_deref () . expect ("owner should have opaque type") . as_ref () . map_bound (| it | & * it . impl_traits [idx] . predicates) } }
    };
}

impl_125!();