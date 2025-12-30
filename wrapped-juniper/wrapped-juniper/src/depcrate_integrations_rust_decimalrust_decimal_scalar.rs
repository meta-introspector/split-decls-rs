// Generated macro for rust_decimal_scalar (module)
macro_rules! Depcrate_integrations_rust_decimalrust_decimal_scalar {
() => {
// Module: crate::integrations::rust_decimal
// Provides: {"rust_decimal_scalar"}
// Dependencies: {}
mod rust_decimal_scalar { use super :: Decimal ; use crate :: { Scalar , ScalarValue } ; pub (super) fn from_input (v : & Scalar < impl ScalarValue >) -> Result < Decimal , Box < str > > { if let Some (i) = v . try_to_int () { Ok (Decimal :: from (i)) } else if let Some (f) = v . try_to_float () { Decimal :: try_from (f) . map_err (| e | format ! ("Failed to parse `Decimal` from `Float`: {e}") . into ()) } else { v . try_to :: < & str > () . map_err (| e | e . to_string () . into ()) . and_then (| s | { s . parse :: < Decimal > () . map_err (| e | format ! ("Failed to parse `Decimal` from `String`: {e}") . into ()) }) } } }
};
}
