macro_rules! deps {
    () => {
        ImplTraitLoweringState!();
        HirDatabase!();
        ImplTraitLoweringMode!();
        LifetimeElisionKind!();
        TyLoweringContext!();
    };
}

macro_rules! impl_798 {
    () => {
        deps!();
        impl < 'db , 'a > TyLoweringContext < 'db , 'a > { pub fn new (db : & 'db dyn HirDatabase , resolver : & 'a Resolver < 'db > , store : & 'a ExpressionStore , def : GenericDefId , lifetime_elision : LifetimeElisionKind < 'db > ,) -> Self { let impl_trait_mode = ImplTraitLoweringState :: new (ImplTraitLoweringMode :: Disallowed) ; let in_binders = DebruijnIndex :: ZERO ; Self { db , interner : DbInterner :: new_with (db , Some (resolver . krate ()) , None) , resolver , def , generics : Default :: default () , store , in_binders , impl_trait_mode , unsized_types : FxHashSet :: default () , diagnostics : Vec :: new () , lifetime_elision , lowering_param_default : None , } } pub (crate) fn set_lifetime_elision (& mut self , lifetime_elision : LifetimeElisionKind < 'db >) { self . lifetime_elision = lifetime_elision ; } pub (crate) fn with_debruijn < T > (& mut self , debruijn : DebruijnIndex , f : impl FnOnce (& mut TyLoweringContext < 'db , '_ >) -> T ,) -> T { let old_debruijn = mem :: replace (& mut self . in_binders , debruijn) ; let result = f (self) ; self . in_binders = old_debruijn ; result } pub (crate) fn with_shifted_in < T > (& mut self , debruijn : DebruijnIndex , f : impl FnOnce (& mut TyLoweringContext < 'db , '_ >) -> T ,) -> T { self . with_debruijn (self . in_binders . shifted_in (debruijn . as_u32 ()) , f) } pub (crate) fn with_impl_trait_mode (self , impl_trait_mode : ImplTraitLoweringMode) -> Self { Self { impl_trait_mode : ImplTraitLoweringState :: new (impl_trait_mode) , .. self } } pub (crate) fn impl_trait_mode (& mut self , impl_trait_mode : ImplTraitLoweringMode) -> & mut Self { self . impl_trait_mode = ImplTraitLoweringState :: new (impl_trait_mode) ; self } pub (crate) fn lowering_param_default (& mut self , index : u32) { self . lowering_param_default = Some (index) ; } pub (crate) fn push_diagnostic (& mut self , type_ref : TypeRefId , kind : TyLoweringDiagnosticKind) { self . diagnostics . push (TyLoweringDiagnostic { source : type_ref , kind }) ; } }
    };
}

impl_798!()