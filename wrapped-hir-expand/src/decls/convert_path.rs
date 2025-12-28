macro_rules! deps {
    () => {
        ModPath!();
        PathKind!();
        ExpandDatabase!();
        Name!();
    };
}

macro_rules! convert_path {
    () => {
        deps!();
        fn convert_path (db : & dyn ExpandDatabase , path : ast :: Path , span_for_range : & mut dyn FnMut (:: tt :: TextRange) -> SyntaxContext ,) -> Option < ModPath > { let mut segments = path . segments () ; let segment = & segments . next () ? ; let handle_super_kw = & mut | init_deg | { let mut deg = init_deg ; let mut next_segment = None ; for segment in segments . by_ref () { match segment . kind () ? { ast :: PathSegmentKind :: SuperKw => deg += 1 , ast :: PathSegmentKind :: Name (name) => { next_segment = Some (name . as_name ()) ; break ; } ast :: PathSegmentKind :: Type { .. } | ast :: PathSegmentKind :: SelfTypeKw | ast :: PathSegmentKind :: SelfKw | ast :: PathSegmentKind :: CrateKw => return None , } } Some (ModPath :: from_segments (PathKind :: Super (deg) , next_segment)) } ; let mut mod_path = match segment . kind () ? { ast :: PathSegmentKind :: Name (name_ref) => { if name_ref . text () == "$crate" { ModPath :: from_kind (resolve_crate_root (db , span_for_range (name_ref . syntax () . text_range ())) . map (PathKind :: DollarCrate) . unwrap_or (PathKind :: Crate) ,) } else { let mut res = ModPath :: from_kind (segment . coloncolon_token () . map_or (PathKind :: Plain , | _ | PathKind :: Abs) ,) ; res . segments . push (name_ref . as_name ()) ; res } } ast :: PathSegmentKind :: SelfTypeKw => { ModPath :: from_segments (PathKind :: Plain , Some (Name :: new_symbol_root (sym :: Self_))) } ast :: PathSegmentKind :: CrateKw => ModPath :: from_segments (PathKind :: Crate , iter :: empty ()) , ast :: PathSegmentKind :: SelfKw => handle_super_kw (0) ? , ast :: PathSegmentKind :: SuperKw => handle_super_kw (1) ? , ast :: PathSegmentKind :: Type { .. } => { return None ; } } ; for segment in segments { let name = match segment . kind () ? { ast :: PathSegmentKind :: Name (name) => name . as_name () , _ => return None , } ; mod_path . segments . push (name) ; } if mod_path . segments . len () == 1 && mod_path . kind == PathKind :: Plain && let Some (_macro_call) = path . syntax () . parent () . and_then (ast :: MacroCall :: cast) { let syn_ctx = span_for_range (segment . syntax () . text_range ()) ; if let Some (macro_call_id) = syn_ctx . outer_expn (db) && db . lookup_intern_macro_call (macro_call_id . into ()) . def . local_inner { mod_path . kind = match resolve_crate_root (db , syn_ctx) { Some (crate_root) => PathKind :: DollarCrate (crate_root) , None => PathKind :: Crate , } } } Some (mod_path) }
    };
}

convert_path!();