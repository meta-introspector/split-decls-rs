// Generated macro for AuxParamsAttr (struct)
macro_rules! Depcrate_significant_drop_tighteningAuxParamsAttr {
() => {
// Module: crate::significant_drop_tightening
// Provides: {"AuxParamsAttr"}
// Dependencies: {}
# [doc = " Auxiliary parameters used on expression created with `#[has_significant_drop]`."] # [derive (Debug)] struct AuxParamsAttr { # [doc = " The number of times `#[has_significant_drop]` was referenced."] counter : usize , # [doc = " If an expensive expression follows the last use of anything marked with"] # [doc = " `#[has_significant_drop]`."] has_expensive_expr_after_last_attr : bool , # [doc = " The identifier of the block that involves the first `#[has_significant_drop]`."] first_block_hir_id : HirId , # [doc = " The span of the block that involves the first `#[has_significant_drop]`."] first_block_span : Span , # [doc = " The binding or variable that references the initial construction of the type marked with"] # [doc = " `#[has_significant_drop]`."] first_bind_ident : Option < Ident > , # [doc = " Similar to `init_bind_ident` but encompasses the right-hand method call."] first_method_span : Span , # [doc = " Similar to `init_bind_ident` but encompasses the whole contained statement."] first_stmt_span : Span , # [doc = " The last visited binding or variable span within a block that had any referenced inner type"] # [doc = " marked with `#[has_significant_drop]`."] last_bind_ident : Option < Ident > , # [doc = " Similar to `last_bind_span` but encompasses the right-hand method call."] last_method_span : Span , # [doc = " Similar to `last_bind_span` but encompasses the whole contained statement."] last_stmt_span : Span , }
};
}
