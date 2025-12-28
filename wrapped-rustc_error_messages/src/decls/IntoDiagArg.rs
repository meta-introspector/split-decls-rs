macro_rules! deps {
    () => {
        DiagArgValue!();
        DiagArg!();
    };
}

macro_rules! IntoDiagArg {
    () => {
        deps!();
        # [doc = " Converts a value of a type into a `DiagArg` (typically a field of an `Diag` struct)."] # [doc = " Implemented as a custom trait rather than `From` so that it is implemented on the type being"] # [doc = " converted rather than on `DiagArgValue`, which enables types from other `rustc_*` crates to"] # [doc = " implement this."] pub trait IntoDiagArg { # [doc = " Convert `Self` into a `DiagArgValue` suitable for rendering in a diagnostic."] # [doc = ""] # [doc = " It takes a `path` where \"long values\" could be written to, if the `DiagArgValue` is too big"] # [doc = " for displaying on the terminal. This path comes from the `Diag` itself. When rendering"] # [doc = " values that come from `TyCtxt`, like `Ty<'_>`, they can use `TyCtxt::short_string`. If a"] # [doc = " value has no shortening logic that could be used, the argument can be safely ignored."] fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue ; }
    };
}

IntoDiagArg!()