// Generated macro for is_union_with_two_non_zst_fields (function)
macro_rules! Depcrate_default_union_representationis_union_with_two_non_zst_fields {
() => {
// Module: crate::default_union_representation
// Provides: {"is_union_with_two_non_zst_fields"}
// Dependencies: {}
# [doc = " Returns true if the given item is a union with at least two non-ZST fields."] # [doc = " (ZST fields having an arbitrary offset is completely inconsequential, and"] # [doc = " if there is only one field left after ignoring ZST fields then the offset"] # [doc = " of that field does not matter either.)"] fn is_union_with_two_non_zst_fields < 'tcx > (cx : & LateContext < 'tcx > , item : & Item < 'tcx >) -> bool { if let ItemKind :: Union (..) = & item . kind && let ty :: Adt (adt_def , args) = cx . tcx . type_of (item . owner_id) . instantiate_identity () . kind () { adt_def . all_fields () . filter (| f | ! is_zst (cx , f , args)) . count () >= 2 } else { false } }
};
}
