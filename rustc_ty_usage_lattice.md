# rustc_ty Usage Lattice (4-deep)

## Depth 0 (284 nodes)

- **ty::InstanceKind::CloneShim** (used by 2 nodes)
  - Used by: rustc_mir_transform::ty::InstanceKind::CloneShim, rustc_ty_utils::ty::InstanceKind::CloneShim
- **ty::CastKind** (used by 25 nodes)
  - Used by: rustc_public::CastKind, rustc_middle::CastKind::As, rustc_middle::mir::CastKind::FloatToFloat, rustc_parse::UnsafeBinderCastKind::Unwrap, rustc_ty_utils::CastKind::As (+20 more)
- **ty::vtable_allocation_provider** (used by 0 nodes)
- **ty::TyMaybeWithLayout::Ty** (used by 2 nodes)
  - Used by: rustc_middle::TyMaybeWithLayout::Ty, rustc_middle::TyMaybeWithLayout::TyAndLayout
- **ty::RelateResult** (used by 7 nodes)
  - Used by: rustc_middle::rustc_type_ir::relate::RelateResult, rustc_middle::RelateResult, rustc_infer::RelateResult, rustc_type_ir::RelateResult, rustc_lint::RelateResult (+2 more)
- **ty::true_significant_drop_ty** (used by 1 nodes)
  - Used by: rustc_middle::true_significant_drop_ty
- **ty::PolyExistentialTraitRef** (used by 5 nodes)
  - Used by: rustc_hir_typeck::ty::PolyExistentialTraitRef, rustc_lint::PolyExistentialTraitRef, rustc_middle::ty::PolyExistentialTraitRef, rustc_infer::ty::PolyExistentialTraitRef, rustc_const_eval::ty::PolyExistentialTraitRef
- **ty::UserTypeKind::TypeOf** (used by 1 nodes)
  - Used by: rustc_hir_typeck::ty::UserTypeKind::TypeOf
- **ty::TraitDef** (used by 3 nodes)
  - Used by: rustc_public::TraitDef, rustc_middle::TraitDef, rustc_hir_analysis::ty::TraitDef
- **ty::Rust2024IncompatiblePatInfo** (used by 1 nodes)
  - Used by: rustc_middle::Rust2024IncompatiblePatInfo
  ... and 274 more nodes

## Depth 1 (2369 nodes)

- **rustc_borrowck::constraint_conversion::ConstraintConversion::new** (used by 0 nodes)
- **rustc_parse::PredicateKindOrStructBody** (used by 0 nodes)
- **rustc_middle::AdtFlags::IS_PHANTOM_DATA** (used by 0 nodes)
- **rustc_mir_transform::ty::AdtDef** (used by 0 nodes)
- **rustc_type_ir::SimplifiedType::Foreign** (used by 0 nodes)
- **rustc_hir_analysis::ExplicitPredicatesMap::new** (used by 0 nodes)
- **rustc_hir_typeck::GatherLocalsVisitor::gather_from_local** (used by 0 nodes)
- **rustc_trait_selection::ObligationCauseCode::WhereClause** (used by 0 nodes)
- **rustc_next_trait_solver::Region::new_static** (used by 0 nodes)
- **rustc_middle::TypeError::Mismatch** (used by 0 nodes)
  ... and 2359 more nodes

