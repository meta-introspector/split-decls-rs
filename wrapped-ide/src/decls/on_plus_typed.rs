macro_rules! on_plus_typed {
    () => {
        fn on_plus_typed (file : & SourceFile , offset : TextSize) -> Option < TextEdit > { let plus_token = file . syntax () . token_at_offset (offset) . right_biased () ? ; if plus_token . kind () != SyntaxKind :: PLUS { return None ; } let mut ancestors = plus_token . parent_ancestors () ; ancestors . next () . and_then (ast :: TypeBoundList :: cast) ? ; let trait_type = ancestors . next () . and_then (< Either < ast :: DynTraitType , ast :: ImplTraitType > > :: cast) ? ; let kind = ancestors . next () ? . kind () ; if ast :: RefType :: can_cast (kind) || ast :: PtrType :: can_cast (kind) || ast :: RetType :: can_cast (kind) { let mut builder = TextEdit :: builder () ; builder . insert (trait_type . syntax () . text_range () . start () , "(" . to_owned ()) ; builder . insert (trait_type . syntax () . text_range () . end () , ")" . to_owned ()) ; Some (builder . finish ()) } else { None } }
    };
}

on_plus_typed!()