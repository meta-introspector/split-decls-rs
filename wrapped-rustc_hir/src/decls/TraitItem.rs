macro_rules! deps {
    () => {
        Generics!();
        TraitItemKind!();
        Defaultness!();
    };
}

macro_rules! TraitItem {
    () => {
        deps!();
        # [doc = " Represents an item declaration within a trait declaration,"] # [doc = " possibly including a default implementation. A trait item is"] # [doc = " either required (meaning it doesn't have an implementation, just a"] # [doc = " signature) or provided (meaning it has a default implementation)."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct TraitItem < 'hir > { pub ident : Ident , pub owner_id : OwnerId , pub generics : & 'hir Generics < 'hir > , pub kind : TraitItemKind < 'hir > , pub span : Span , pub defaultness : Defaultness , pub has_delayed_lints : bool , }
    };
}

TraitItem!();