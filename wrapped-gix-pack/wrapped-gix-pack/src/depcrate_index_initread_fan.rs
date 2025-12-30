// Generated macro for read_fan (function)
macro_rules! Depcrate_index_initread_fan {
() => {
// Module: crate::index::init
// Provides: {"read_fan"}
// Dependencies: {}
fn read_fan (d : & [u8]) -> ([u32 ; FAN_LEN] , usize) { assert ! (d . len () >= FAN_LEN * N32_SIZE) ; let mut fan = [0 ; FAN_LEN] ; for (c , f) in d . chunks_exact (N32_SIZE) . zip (fan . iter_mut ()) { * f = crate :: read_u32 (c) ; } (fan , FAN_LEN * N32_SIZE) }
};
}
