macro_rules! FieldInfo {
    () => {
        # [doc = " Summary of the relevant parts of a struct/enum field."] pub (crate) struct FieldInfo { pub span : Span , # [doc = " None for tuple structs/normal enum variants, Some for normal"] # [doc = " structs/struct enum variants."] pub name : Option < Ident > , # [doc = " The expression corresponding to this field of `self`"] # [doc = " (specifically, a reference to it)."] pub self_expr : Box < Expr > , # [doc = " The expressions corresponding to references to this field in"] # [doc = " the other selflike arguments."] pub other_selflike_exprs : Vec < Box < Expr > > , pub maybe_scalar : bool , }
    };
}

FieldInfo!()