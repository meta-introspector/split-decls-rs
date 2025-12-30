// Generated macro for format_field (function)
macro_rules! Depcrate_snapshotformat_field {
() => {
// Module: crate::snapshot
// Provides: {"format_field"}
// Dependencies: {}
fn format_field (val : & Operand , ty : & Type) -> Option < TokenStream > { if ! is_printable (ty) { return None ; } let format = match ty { Type :: Option (ty) => { if let Some (format) = format_field (& Borrowed (quote ! (_val)) , ty) { let ty = rust_type (ty) ; let val = val . ref_tokens () ; quote ! ({ # [derive (RefCast)] # [repr (transparent)] struct Print (Option <# ty >) ; impl Debug for Print { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match & self . 0 { Some (_val) => { formatter . write_str ("Some(") ?; Debug :: fmt (# format , formatter) ?; formatter . write_str (")") ?; Ok (()) } None => formatter . write_str ("None") , } } } Print :: ref_cast (# val) }) } else { let val = val . tokens () ; quote ! { & super :: Option { present : # val . is_some () } } } } Type :: Tuple (ty) => { let printable : Vec < TokenStream > = ty . iter () . enumerate () . filter_map (| (i , ty) | { let index = Index :: from (i) ; let val = val . tokens () ; let inner = Owned (quote ! (# val .# index)) ; format_field (& inner , ty) }) . collect () ; if printable . len () == 1 { printable . into_iter () . next () . unwrap () } else { quote ! { & (# (# printable) ,*) } } } _ => { let val = val . ref_tokens () ; quote ! { Lite (# val) } } } ; Some (format) }
};
}
