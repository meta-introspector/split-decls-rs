macro_rules! deps {
    () => {
        ImplItemKind!();
        ImplItemImplKind!();
        Generics!();
        Impl!();
    };
}

macro_rules! ImplItem {
    () => {
        deps!();
        # [doc = " Represents an associated item within an impl block."] # [doc = ""] # [doc = " Refer to [`Impl`] for an impl block declaration."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct ImplItem < 'hir > { pub ident : Ident , pub owner_id : OwnerId , pub generics : & 'hir Generics < 'hir > , pub kind : ImplItemKind < 'hir > , pub impl_kind : ImplItemImplKind , pub span : Span , pub has_delayed_lints : bool , }
    };
}

ImplItem!()