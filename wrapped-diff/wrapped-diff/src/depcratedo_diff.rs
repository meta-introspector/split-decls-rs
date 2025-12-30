// Generated macro for do_diff (function)
macro_rules! Depcratedo_diff {
() => {
// Module: crate
// Provides: {"do_diff"}
// Dependencies: {}
fn do_diff < 'a , T , F , U > (left : & 'a [T] , right : & 'a [T] , mapper : F) -> Vec < Result < U > > where T : PartialEq , F : Fn (& 'a T) -> U , { let leading_equals = left . iter () . zip (right . iter ()) . take_while (| (l , r) | l == r) . count () ; let trailing_equals = left [leading_equals ..] . iter () . rev () . zip (right [leading_equals ..] . iter () . rev ()) . take_while (| (l , r) | l == r) . count () ; let mut diff = Vec :: with_capacity (left . len () . max (right . len ())) ; diff . extend (left [.. leading_equals] . iter () . zip (& right [.. leading_equals]) . map (| (l , r) | Result :: Both (mapper (l) , mapper (r))) ,) ; do_naive_diff (& left [leading_equals .. left . len () - trailing_equals] , & right [leading_equals .. right . len () - trailing_equals] , & mapper , & mut diff ,) ; diff . extend (left [left . len () - trailing_equals ..] . iter () . zip (& right [right . len () - trailing_equals ..]) . map (| (l , r) | Result :: Both (mapper (l) , mapper (r))) ,) ; diff }
};
}
