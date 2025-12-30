// Generated macro for WhereEqPredicate (struct)
macro_rules! Depcrate_astWhereEqPredicate {
() => {
// Module: crate::ast
// Provides: {"WhereEqPredicate"}
// Dependencies: {}
# [doc = " An equality predicate (unsupported)."] # [doc = ""] # [doc = " E.g., `T = int`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct WhereEqPredicate { pub lhs_ty : Box < Ty > , pub rhs_ty : Box < Ty > , }
};
}
