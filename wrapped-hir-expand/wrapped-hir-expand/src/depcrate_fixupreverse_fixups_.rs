// Generated macro for reverse_fixups_ (function)
macro_rules! Depcrate_fixupreverse_fixups_ {
() => {
// Module: crate::fixup
// Provides: {"reverse_fixups_"}
// Dependencies: {}
fn reverse_fixups_ (tt : & mut TopSubtree , undo_info : & [TopSubtree]) { let mut tts = std :: mem :: take (& mut tt . 0) . into_vec () ; transform_tt (& mut tts , | tt | match tt { tt :: TokenTree :: Leaf (leaf) => { let span = leaf . span () ; let is_real_leaf = span . anchor . ast_id != FIXUP_DUMMY_AST_ID ; let is_replaced_node = span . range . end () == FIXUP_DUMMY_RANGE_END ; if ! is_real_leaf && ! is_replaced_node { return TransformTtAction :: remove () ; } if ! is_real_leaf { let original = & undo_info [u32 :: from (leaf . span () . range . start ()) as usize] ; TransformTtAction :: ReplaceWith (original . view () . strip_invisible ()) } else { TransformTtAction :: Keep } } tt :: TokenTree :: Subtree (tt) => { if tt . delimiter . close . anchor . ast_id == FIXUP_DUMMY_AST_ID || tt . delimiter . open . anchor . ast_id == FIXUP_DUMMY_AST_ID { return TransformTtAction :: remove () ; } TransformTtAction :: Keep } }) ; tt . 0 = tts . into_boxed_slice () ; }
};
}
