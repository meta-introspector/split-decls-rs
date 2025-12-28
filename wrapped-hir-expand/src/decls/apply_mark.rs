macro_rules! deps {
    () => {
        ExpandDatabase!();
        MacroCallId!();
    };
}

macro_rules! apply_mark {
    () => {
        deps!();
        pub (super) fn apply_mark (db : & dyn ExpandDatabase , ctxt : span :: SyntaxContext , call_id : span :: MacroCallId , transparency : Transparency , edition : Edition ,) -> SyntaxContext { if transparency == Transparency :: Opaque { return apply_mark_internal (db , ctxt , call_id , transparency , edition) ; } let call_site_ctxt = db . lookup_intern_macro_call (call_id . into ()) . ctxt ; let mut call_site_ctxt = if transparency == Transparency :: SemiTransparent { call_site_ctxt . normalize_to_macros_2_0 (db) } else { call_site_ctxt . normalize_to_macro_rules (db) } ; if call_site_ctxt . is_root () { return apply_mark_internal (db , ctxt , call_id , transparency , edition) ; } for (call_id , transparency) in ctxt . marks (db) { call_site_ctxt = apply_mark_internal (db , call_site_ctxt , call_id , transparency , edition) ; } apply_mark_internal (db , call_site_ctxt , call_id , transparency , edition) }
    };
}

apply_mark!()