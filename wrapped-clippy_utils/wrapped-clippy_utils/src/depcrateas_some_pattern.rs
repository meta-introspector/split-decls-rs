// Generated macro for as_some_pattern (function)
macro_rules! Depcrateas_some_pattern {
() => {
// Module: crate
// Provides: {"as_some_pattern"}
// Dependencies: {}
# [doc = " If `pat` is:"] # [doc = " - `Some(inner)`, returns `inner`"] # [doc = "    - it will _usually_ contain just one element, but could have two, given patterns like"] # [doc = "      `Some(inner, ..)` or `Some(.., inner)`"] # [doc = " - `Some`, returns `[]`"] # [doc = " - otherwise, returns `None`"] pub fn as_some_pattern < 'a , 'hir > (cx : & LateContext < '_ > , pat : & 'a Pat < 'hir >) -> Option < & 'a [Pat < 'hir >] > { if let PatKind :: TupleStruct (ref qpath , inner , _) = pat . kind && cx . qpath_res (qpath , pat . hir_id) . ctor_parent (cx) . is_lang_item (cx , OptionSome) { Some (inner) } else { None } }
};
}
