macro_rules! deps {
    () => {
        BorrowckInferCtxt!();
        MirTypeckRegionConstraints!();
        RegionCtxt!();
        UniversalRegionRelations!();
        DeferredOpaqueTypeError!();
    };
}

macro_rules! compute_concrete_opaque_types {
    () => {
        deps!();
        # [doc = " This computes the actual hidden types of the opaque types and maps them to their"] # [doc = " definition sites. Outside of registering the computed concrete types this function"] # [doc = " does not mutate the current borrowck state."] # [doc = ""] # [doc = " While it may fail to infer the hidden type and return errors, we always apply"] # [doc = " the computed concrete hidden type to all opaque type uses to check whether they"] # [doc = " are correct. This is necessary to support non-defining uses of opaques in their"] # [doc = " defining scope."] # [doc = ""] # [doc = " It also means that this whole function is not really soundness critical as we"] # [doc = " recheck all uses of the opaques regardless."] pub (crate) fn compute_concrete_opaque_types < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , universal_region_relations : & Frozen < UniversalRegionRelations < 'tcx > > , constraints : & MirTypeckRegionConstraints < 'tcx > , location_map : Rc < DenseLocationMap > , concrete_opaque_types : & mut ConcreteOpaqueTypes < 'tcx > , opaque_types : & [(OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx >)] ,) -> Vec < DeferredOpaqueTypeError < 'tcx > > { let mut errors = Vec :: new () ; let mut rcx = RegionCtxt :: new (infcx , universal_region_relations , location_map , constraints) ; let defining_uses = collect_defining_uses (& mut rcx , concrete_opaque_types , opaque_types , & mut errors) ; apply_member_constraints (& mut rcx , & defining_uses) ; compute_concrete_types_from_defining_uses (& rcx , concrete_opaque_types , & defining_uses , & mut errors ,) ; errors }
    };
}

compute_concrete_opaque_types!();