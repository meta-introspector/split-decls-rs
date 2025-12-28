macro_rules! deps {
    () => {
        ForeignItemKind!();
    };
}

macro_rules! ForeignItem {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct ForeignItem < 'hir > { pub ident : Ident , pub kind : ForeignItemKind < 'hir > , pub owner_id : OwnerId , pub span : Span , pub vis_span : Span , pub has_delayed_lints : bool , }
    };
}

ForeignItem!();