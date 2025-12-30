// Generated macro for size_asserts (module)
macro_rules! Depcrate_astsize_asserts {
() => {
// Module: crate::ast
// Provides: {"size_asserts"}
// Dependencies: {}
# [cfg (target_pointer_width = "64")] mod size_asserts { use rustc_data_structures :: static_assert_size ; use super :: * ; static_assert_size ! (AssocItem , 80) ; static_assert_size ! (AssocItemKind , 16) ; static_assert_size ! (Attribute , 32) ; static_assert_size ! (Block , 32) ; static_assert_size ! (Expr , 72) ; static_assert_size ! (ExprKind , 40) ; static_assert_size ! (Fn , 184) ; static_assert_size ! (ForeignItem , 80) ; static_assert_size ! (ForeignItemKind , 16) ; static_assert_size ! (GenericArg , 24) ; static_assert_size ! (GenericBound , 88) ; static_assert_size ! (Generics , 40) ; static_assert_size ! (Impl , 64) ; static_assert_size ! (Item , 144) ; static_assert_size ! (ItemKind , 80) ; static_assert_size ! (LitKind , 24) ; static_assert_size ! (Local , 96) ; static_assert_size ! (MetaItemLit , 40) ; static_assert_size ! (Param , 40) ; static_assert_size ! (Pat , 80) ; static_assert_size ! (PatKind , 56) ; static_assert_size ! (Path , 24) ; static_assert_size ! (PathSegment , 24) ; static_assert_size ! (Stmt , 32) ; static_assert_size ! (StmtKind , 16) ; static_assert_size ! (TraitImplHeader , 80) ; static_assert_size ! (Ty , 64) ; static_assert_size ! (TyKind , 40) ; }
};
}
