// Generated macro for gt_off (macro)
macro_rules! Depcrate_gzgt_off {
() => {
// Module: crate::gz
// Provides: {"gt_off"}
// Dependencies: {}
macro_rules ! gt_off { ($ x : expr) => { core :: mem :: size_of_val (&$ x) == core :: mem :: size_of ::< i64 > () && $ x as usize > i64 :: MAX as usize } ; }
};
}
