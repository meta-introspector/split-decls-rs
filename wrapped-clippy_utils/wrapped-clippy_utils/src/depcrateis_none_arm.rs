// Generated macro for is_none_arm (function)
macro_rules! Depcrateis_none_arm {
() => {
// Module: crate
// Provides: {"is_none_arm"}
// Dependencies: {}
# [doc = " Checks if `arm` has the form `None => None`."] pub fn is_none_arm (cx : & LateContext < '_ > , arm : & Arm < '_ >) -> bool { is_none_pattern (cx , arm . pat) && matches ! (peel_blocks (arm . body) . kind , ExprKind :: Path (qpath) if cx . qpath_res (& qpath , arm . body . hir_id) . ctor_parent (cx) . is_lang_item (cx , OptionNone)) }
};
}
