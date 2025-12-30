// Generated macro for parse_bounds (function)
macro_rules! Depcrate_momoparse_bounds {
() => {
// Module: crate::momo
// Provides: {"parse_bounds"}
// Dependencies: {}
fn parse_bounds (bounds : & Punctuated < TypeParamBound , Token ! [+] >) -> Option < Conversion > { if bounds . len () != 1 { return None ; } if let TypeParamBound :: Trait (ref tb) = bounds . first () . unwrap () { if let Some (seg) = tb . path . segments . iter () . next_back () { if let PathArguments :: AngleBracketed (ref gen_args) = seg . arguments { if let GenericArgument :: Type (_) = gen_args . args . first () . unwrap () { if seg . ident == "Into" { return Some (Conversion :: Into) ; } else if seg . ident == "AsRef" { return Some (Conversion :: AsRef) ; } else if seg . ident == "AsMut" { return Some (Conversion :: AsMut) ; } } } } } None }
};
}
