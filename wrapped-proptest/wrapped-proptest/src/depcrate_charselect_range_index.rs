// Generated macro for select_range_index (function)
macro_rules! Depcrate_charselect_range_index {
() => {
// Module: crate::char
// Provides: {"select_range_index"}
// Dependencies: {}
fn select_range_index (rnd : & mut impl Rng , special : & [char] , preferred : & [CharRange] , ranges : & [CharRange] ,) -> (u32 , u32) { fn in_range (ranges : & [CharRange] , ch : char) -> Option < (u32 , u32) > { ranges . iter () . find (| r | ch >= * r . start () && ch <= * r . end ()) . map (| r | (* r . start () as u32 , ch as u32 - * r . start () as u32)) } if ! special . is_empty () && rnd . random () { let s = special [rnd . random_range (0 .. special . len ())] ; if let Some (ret) = in_range (ranges , s) { return ret ; } } if ! preferred . is_empty () && rnd . random () { let range = preferred [rnd . random_range (0 .. preferred . len ())] . clone () ; if let Some (ch) = :: core :: char :: from_u32 (rnd . random_range (* range . start () as u32 .. * range . end () as u32 + 1) ,) { if let Some (ret) = in_range (ranges , ch) { return ret ; } } } for _ in 0 .. 65_536 { let range = ranges [rnd . random_range (0 .. ranges . len ())] . clone () ; if let Some (ch) = :: core :: char :: from_u32 (rnd . random_range (* range . start () as u32 .. * range . end () as u32 + 1) ,) { return (* range . start () as u32 , ch as u32 - * range . start () as u32) ; } } (* ranges [0] . start () as u32 , 0) }
};
}
