macro_rules! InferenceLiteralEraser {
    () => {
        # [doc = " Replace `{integer}` with `i32` and `{float}` with `f64`."] # [doc = " Used only for diagnostics."] struct InferenceLiteralEraser < 'tcx > { tcx : TyCtxt < 'tcx > , }
    };
}

InferenceLiteralEraser!()