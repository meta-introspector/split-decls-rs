macro_rules! deps {
    () => {
        CodegenUnitDebugContext!();
        CodegenCx!();
    };
}

macro_rules! debug_context {
    () => {
        deps!();
        # [inline] pub (crate) fn debug_context < 'a , 'll , 'tcx > (cx : & 'a CodegenCx < 'll , 'tcx > ,) -> & 'a CodegenUnitDebugContext < 'll , 'tcx > { cx . dbg_cx . as_ref () . unwrap () }
    };
}

debug_context!();