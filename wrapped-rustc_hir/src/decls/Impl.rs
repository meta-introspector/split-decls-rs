macro_rules! deps {
    () => {
        ImplItem!();
        TraitImplHeader!();
        Ty!();
        ImplItemId!();
        Generics!();
    };
}

macro_rules! Impl {
    () => {
        deps!();
        # [doc = " Represents an impl block declaration."] # [doc = ""] # [doc = " E.g., `impl $Type { .. }` or `impl $Trait for $Type { .. }`"] # [doc = " Refer to [`ImplItem`] for an associated item within an impl block."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Impl < 'hir > { pub generics : & 'hir Generics < 'hir > , pub of_trait : Option < & 'hir TraitImplHeader < 'hir > > , pub self_ty : & 'hir Ty < 'hir > , pub items : & 'hir [ImplItemId] , }
    };
}

Impl!()