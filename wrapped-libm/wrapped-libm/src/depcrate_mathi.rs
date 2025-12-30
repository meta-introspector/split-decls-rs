// Generated macro for i (macro)
macro_rules! Depcrate_mathi {
() => {
// Module: crate::math
// Provides: {"i"}
// Dependencies: {}
# [cfg (debug_assertions)] macro_rules ! i { ($ array : expr , $ index : expr) => { *$ array . get ($ index) . unwrap () } ; ($ array : expr , $ index : expr , = , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () = $ rhs ; } ; ($ array : expr , $ index : expr , -= , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () -= $ rhs ; } ; ($ array : expr , $ index : expr , += , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () += $ rhs ; } ; ($ array : expr , $ index : expr , &= , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () &= $ rhs ; } ; ($ array : expr , $ index : expr , == , $ rhs : expr) => { *$ array . get_mut ($ index) . unwrap () == $ rhs } ; }
};
}
