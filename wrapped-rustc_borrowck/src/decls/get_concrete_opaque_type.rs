macro_rules! get_concrete_opaque_type {
    () => {
        fn get_concrete_opaque_type < 'tcx > (concrete_opaque_types : & ConcreteOpaqueTypes < 'tcx > , def_id : LocalDefId ,) -> Option < EarlyBinder < 'tcx , OpaqueHiddenType < 'tcx > > > { concrete_opaque_types . 0 . get (& def_id) . map (| ty | EarlyBinder :: bind (* ty)) }
    };
}

get_concrete_opaque_type!()