// Generated macro for transform_dom (function)
macro_rules! Depcratetransform_dom {
() => {
// Module: crate
// Provides: {"transform_dom"}
// Dependencies: {}
pub fn transform_dom (s : & str , flipped : bool , elongate : bool , with_markers : bool) -> Cow < '_ , str > { if s . len () == 1 { return s . into () ; } let mut result = Cow :: from (s) ; let mut pos = 0 ; let mut diff = 0 ; for cap in RE_EXCLUDED . captures_iter (s) { let capture = cap . get (0) . unwrap () ; let sub_len = capture . start () - pos ; let range = pos .. capture . start () ; let result_range = pos + diff .. capture . start () + diff ; let sub = & s [range . clone ()] ; let transform_sub = transform (sub , false , true) ; diff += transform_sub . len () - sub_len ; result . to_mut () . replace_range (result_range . clone () , & transform_sub) ; pos = capture . end () ; } let range = pos .. s . len () ; let result_range = pos + diff .. result . len () ; let transform_sub = transform (& s [range] , flipped , elongate) ; result . to_mut () . replace_range (result_range , & transform_sub) ; if with_markers { return Cow :: from ("[") + result + "]" ; } result }
};
}
