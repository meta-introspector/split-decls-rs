macro_rules! rendered_precise_capturing_args {
    () => {
        fn rendered_precise_capturing_args < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> Option < & 'tcx [PreciseCapturingArgKind < Symbol , Symbol >] > { if let Some (ty :: ImplTraitInTraitData :: Trait { opaque_def_id , .. }) = tcx . opt_rpitit_info (def_id . to_def_id ()) { return tcx . rendered_precise_capturing_args (opaque_def_id) ; } tcx . hir_node_by_def_id (def_id) . expect_opaque_ty () . bounds . iter () . find_map (| bound | match bound { hir :: GenericBound :: Use (args , ..) => { Some (& * tcx . arena . alloc_from_iter (args . iter () . map (| arg | match arg { PreciseCapturingArgKind :: Lifetime (_) => { PreciseCapturingArgKind :: Lifetime (arg . name ()) } PreciseCapturingArgKind :: Param (_) => PreciseCapturingArgKind :: Param (arg . name ()) , }))) } _ => None , }) }
    };
}

rendered_precise_capturing_args!();