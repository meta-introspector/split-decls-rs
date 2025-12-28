macro_rules! OpaqueTypeStorage {
    () => {
        # [derive (Default , Debug , Clone)] pub struct OpaqueTypeStorage < 'tcx > { opaque_types : FxIndexMap < OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx > > , duplicate_entries : Vec < (OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx >) > , }
    };
}

OpaqueTypeStorage!()