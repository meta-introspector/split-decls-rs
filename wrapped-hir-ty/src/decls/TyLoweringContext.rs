macro_rules! deps {
    () => {
        HirDatabase!();
        Generics!();
        LifetimeElisionKind!();
        ImplTraitLoweringState!();
    };
}

macro_rules! TyLoweringContext {
    () => {
        deps!();
        # [derive (Debug)] pub struct TyLoweringContext < 'db , 'a > { pub db : & 'db dyn HirDatabase , interner : DbInterner < 'db > , resolver : & 'a Resolver < 'db > , store : & 'a ExpressionStore , def : GenericDefId , generics : OnceCell < Generics > , in_binders : DebruijnIndex , impl_trait_mode : ImplTraitLoweringState < 'db > , # [doc = " Tracks types with explicit `?Sized` bounds."] pub (crate) unsized_types : FxHashSet < Ty < 'db > > , pub (crate) diagnostics : Vec < TyLoweringDiagnostic > , lifetime_elision : LifetimeElisionKind < 'db > , # [doc = " When lowering the defaults for generic params, this contains the index of the currently lowered param."] # [doc = " We disallow referring to later params, or to ADT's `Self`."] lowering_param_default : Option < u32 > , }
    };
}

TyLoweringContext!()