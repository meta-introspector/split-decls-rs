// Generated macro for apply_mark_internal (function)
macro_rules! Depcrate_hygieneapply_mark_internal {
() => {
// Module: crate::hygiene
// Provides: {"apply_mark_internal"}
// Dependencies: {}
fn apply_mark_internal (db : & dyn ExpandDatabase , ctxt : SyntaxContext , call_id : MacroCallId , transparency : Transparency , edition : Edition ,) -> SyntaxContext { let call_id = Some (call_id) ; let mut opaque = ctxt . opaque (db) ; let mut opaque_and_semitransparent = ctxt . opaque_and_semitransparent (db) ; if transparency >= Transparency :: Opaque { let parent = opaque ; opaque = SyntaxContext :: new (db , call_id , transparency , edition , parent , identity , identity) ; } if transparency >= Transparency :: SemiTransparent { let parent = opaque_and_semitransparent ; opaque_and_semitransparent = SyntaxContext :: new (db , call_id , transparency , edition , parent , | _ | opaque , identity) ; } let parent = ctxt ; SyntaxContext :: new (db , call_id , transparency , edition , parent , | _ | opaque , | _ | opaque_and_semitransparent ,) }
};
}
