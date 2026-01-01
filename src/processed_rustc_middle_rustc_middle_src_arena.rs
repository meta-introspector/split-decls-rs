// SRC: ../rust/compiler/rustc_middle/src/arena.rs
/// This higher-order macro declares a list of types which can be allocated by `Arena`.
///
/// Specifying the `decode` modifier will add decode impls for `&T` and `&[T]` where `T` is the type
/// listed. These impls will appear in the implement_ty_decoder! macro.
#[macro_export]
macro_rules! arena_types {
    ($macro:path) => (
        $macro!([
            [] layout: crate::rustc_abi::LayoutData<crate::rustc_abi::FieldIdx, crate::rustc_abi::VariantIdx>,
            [] proxy_coroutine_layout: crate::rustc_middle::mir::CoroutineLayout<'tcx>,
            [] fn_abi: crate::rustc_target::callconv::FnAbi<'tcx, crate::rustc_middle::ty::Ty<'tcx>>,
            // AdtDef are interned and compared by address
            [decode] adt_def: crate::rustc_middle::ty::AdtDefData,
            [] steal_thir: crate::rustc_data_structures::steal::Steal<crate::rustc_middle::thir::Thir<'tcx>>,
            [] steal_mir: crate::rustc_data_structures::steal::Steal<crate::rustc_middle::mir::Body<'tcx>>,
            [decode] mir: crate::rustc_middle::mir::Body<'tcx>,
            [] steal_promoted:
                crate::rustc_data_structures::steal::Steal<
                    crate::rustc_index::IndexVec<
                        crate::rustc_middle::mir::Promoted,
                        crate::rustc_middle::mir::Body<'tcx>
                    >
                >,
            [decode] promoted:
                crate::rustc_index::IndexVec<
                    crate::rustc_middle::mir::Promoted,
                    crate::rustc_middle::mir::Body<'tcx>
                >,
            [decode] typeck_results: crate::rustc_middle::ty::TypeckResults<'tcx>,
            [decode] borrowck_result: crate::rustc_middle::mir::ConcreteOpaqueTypes<'tcx>,
            [] resolver: crate::rustc_data_structures::steal::Steal<(
                crate::rustc_middle::ty::ResolverAstLowering,
                std::sync::Arc<crate::rustc_ast::Crate>,
            )>,
            [] crate_for_resolver: crate::rustc_data_structures::steal::Steal<(crate::rustc_ast::Crate, crate::rustc_ast::AttrVec)>,
            [] resolutions: crate::rustc_middle::ty::ResolverGlobalCtxt,
            [] const_allocs: crate::rustc_middle::mir::interpret::Allocation,
            [] region_scope_tree: crate::rustc_middle::middle::region::ScopeTree,
            // Required for the incremental on-disk cache
            [] mir_keys: crate::rustc_hir::def_id::DefIdSet,
            [] dropck_outlives:
                crate::rustc_middle::infer::canonical::Canonical<'tcx,
                    crate::rustc_middle::infer::canonical::QueryResponse<'tcx,
                        crate::rustc_middle::traits::query::DropckOutlivesResult<'tcx>
                    >
                >,
            [] normalize_canonicalized_projection_ty:
                crate::rustc_middle::infer::canonical::Canonical<'tcx,
                    crate::rustc_middle::infer::canonical::QueryResponse<'tcx,
                        crate::rustc_middle::traits::query::NormalizationResult<'tcx>
                    >
                >,
            [] implied_outlives_bounds:
                crate::rustc_middle::infer::canonical::Canonical<'tcx,
                    crate::rustc_middle::infer::canonical::QueryResponse<'tcx,
                        Vec<crate::rustc_middle::traits::query::OutlivesBound<'tcx>>
                    >
                >,
            [] dtorck_constraint: crate::rustc_middle::traits::query::DropckConstraint<'tcx>,
            [] candidate_step: crate::rustc_middle::traits::query::CandidateStep<'tcx>,
            [] autoderef_bad_ty: crate::rustc_middle::traits::query::MethodAutoderefBadTy<'tcx>,
            [] query_region_constraints: crate::rustc_middle::infer::canonical::QueryRegionConstraints<'tcx>,
            [] type_op_subtype:
                crate::rustc_middle::infer::canonical::Canonical<'tcx,
                    crate::rustc_middle::infer::canonical::QueryResponse<'tcx, ()>
                >,
            [] type_op_normalize_poly_fn_sig:
                crate::rustc_middle::infer::canonical::Canonical<'tcx,
                    crate::rustc_middle::infer::canonical::QueryResponse<'tcx, crate::rustc_middle::ty::PolyFnSig<'tcx>>
                >,
            [] type_op_normalize_fn_sig:
                crate::rustc_middle::infer::canonical::Canonical<'tcx,
                    crate::rustc_middle::infer::canonical::QueryResponse<'tcx, crate::rustc_middle::ty::FnSig<'tcx>>
                >,
            [] type_op_normalize_clause:
                crate::rustc_middle::infer::canonical::Canonical<'tcx,
                    crate::rustc_middle::infer::canonical::QueryResponse<'tcx, crate::rustc_middle::ty::Clause<'tcx>>
                >,
            [] type_op_normalize_ty:
                crate::rustc_middle::infer::canonical::Canonical<'tcx,
                    crate::rustc_middle::infer::canonical::QueryResponse<'tcx, crate::rustc_middle::ty::Ty<'tcx>>
                >,
            [] inspect_probe: crate::rustc_middle::traits::solve::inspect::Probe<crate::rustc_middle::ty::TyCtxt<'tcx>>,
            [] effective_visibilities: crate::rustc_middle::middle::privacy::EffectiveVisibilities,
            [] upvars_mentioned: crate::rustc_data_structures::fx::FxIndexMap<crate::rustc_hir::HirId, crate::rustc_hir::Upvar>,
            [] dyn_compatibility_violations: crate::rustc_middle::traits::DynCompatibilityViolation,
            [] codegen_unit: crate::rustc_middle::mir::mono::CodegenUnit<'tcx>,
            [decode] attribute: crate::rustc_hir::Attribute,
            [] name_set: crate::rustc_data_structures::unord::UnordSet<crate::rustc_span::Symbol>,
            [] autodiff_item: crate::rustc_ast::expand::autodiff_attrs::AutoDiffItem,
            [] ordered_name_set: crate::rustc_data_structures::fx::FxIndexSet<crate::rustc_span::Symbol>,
            [] valtree: crate::rustc_middle::ty::ValTreeKind<'tcx>,
            [] stable_order_of_exportable_impls:
                crate::rustc_data_structures::fx::FxIndexMap<crate::rustc_hir::def_id::DefId, usize>,

            // Note that this deliberately duplicates items in the `crate::rustc_hir::arena`,
            // since we need to allocate this type on both the `rustc_hir` arena
            // (during lowering) and the `rustc_middle` arena (for decoding MIR)
            [decode] asm_template: crate::rustc_ast::InlineAsmTemplatePiece,
            [decode] used_trait_imports: crate::rustc_data_structures::unord::UnordSet<crate::rustc_hir::def_id::LocalDefId>,
            [decode] is_late_bound_map: crate::rustc_data_structures::fx::FxIndexSet<crate::rustc_hir::ItemLocalId>,
            [decode] impl_source: crate::rustc_middle::traits::ImplSource<'tcx, ()>,

            [] dep_kind: crate::rustc_middle::dep_graph::DepKindStruct<'tcx>,

            [decode] trait_impl_trait_tys:
                crate::rustc_data_structures::unord::UnordMap<
                    crate::rustc_hir::def_id::DefId,
                    crate::rustc_middle::ty::EarlyBinder<'tcx, crate::rustc_middle::ty::Ty<'tcx>>
                >,
            [] external_constraints: crate::rustc_middle::traits::solve::ExternalConstraintsData<crate::rustc_middle::ty::TyCtxt<'tcx>>,
            [] predefined_opaques_in_body: crate::rustc_middle::traits::solve::PredefinedOpaquesData<crate::rustc_middle::ty::TyCtxt<'tcx>>,
            [decode] doc_link_resolutions: crate::rustc_hir::def::DocLinkResMap,
            [] stripped_cfg_items: crate::rustc_hir::attrs::StrippedCfgItem,
            [] mod_child: crate::rustc_middle::metadata::ModChild,
            [] features: crate::rustc_feature::Features,
            [decode] specialization_graph: crate::rustc_middle::traits::specialization_graph::Graph,
            [] crate_inherent_impls: crate::rustc_middle::ty::CrateInherentImpls,
            [] hir_owner_nodes: crate::rustc_hir::OwnerNodes<'tcx>,
        ]);
    )
}

arena_types!(rustc_arena::declare_arena);