macro_rules! deps {
    () => {
        TimedOut!();
        BoxError!();
    };
}

macro_rules! cast_to_internal_error {
    () => {
        deps!();
        # [doc = " Converts from external types to reqwest's"] # [doc = " internal equivalents."] # [doc = ""] # [doc = " Currently only is used for `tower::timeout::error::Elapsed`."] # [cfg (not (target_arch = "wasm32"))] pub (crate) fn cast_to_internal_error (error : BoxError) -> BoxError { if error . is :: < tower :: timeout :: error :: Elapsed > () { Box :: new (crate :: error :: TimedOut) as BoxError } else { error } }
    };
}

cast_to_internal_error!();