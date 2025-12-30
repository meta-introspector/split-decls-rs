// Generated macro for bigdecimal_scalar (module)
macro_rules! Depcrate_integrations_bigdecimalbigdecimal_scalar {
() => {
// Module: crate::integrations::bigdecimal
// Provides: {"bigdecimal_scalar"}
// Dependencies: {}
mod bigdecimal_scalar { use super :: BigDecimal ; use crate :: { Scalar , ScalarValue } ; pub (super) fn from_input (v : & Scalar < impl ScalarValue >) -> Result < BigDecimal , Box < str > > { if let Some (i) = v . try_to_int () { Ok (BigDecimal :: from (i)) } else if let Some (f) = v . try_to_float () { let mut buf = ryu :: Buffer :: new () ; buf . format (f) . parse :: < BigDecimal > () . map_err (| e | format ! ("Failed to parse `BigDecimal` from `Float`: {e}") . into ()) } else { v . try_to :: < & str > () . map_err (| e | e . to_string () . into ()) . and_then (| s | { s . parse :: < BigDecimal > () . map_err (| e | { format ! ("Failed to parse `BigDecimal` from `String`: {e}") . into () }) }) } } }
};
}
