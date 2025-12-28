macro_rules! deps {
    () => {
        TraitBoundModifiers!();
        GenericParam!();
        TraitRef!();
    };
}

macro_rules! PolyTraitRef {
    () => {
        deps!();
        # [derive (Clone , Debug , Copy , HashStable_Generic)] pub struct PolyTraitRef < 'hir > { # [doc = " The `'a` in `for<'a> Foo<&'a T>`."] pub bound_generic_params : & 'hir [GenericParam < 'hir >] , # [doc = " The constness and polarity of the trait ref."] # [doc = ""] # [doc = " The `async` modifier is lowered directly into a different trait for now."] pub modifiers : TraitBoundModifiers , # [doc = " The `Foo<&'a T>` in `for<'a> Foo<&'a T>`."] pub trait_ref : TraitRef < 'hir > , pub span : Span , }
    };
}

PolyTraitRef!()