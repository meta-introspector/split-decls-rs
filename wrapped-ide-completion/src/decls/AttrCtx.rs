macro_rules! AttrCtx {
    () => {
        # [derive (Debug , PartialEq , Eq)] pub (crate) struct AttrCtx { pub (crate) kind : AttrKind , pub (crate) annotated_item_kind : Option < SyntaxKind > , pub (crate) derive_helpers : Vec < (Symbol , Symbol) > , }
    };
}

AttrCtx!();