// Generated macro for insert_aligned (function)
macro_rules! Depcrate_arm_linuxinsert_aligned {
() => {
// Module: crate::arm_linux
// Provides: {"insert_aligned"}
// Dependencies: {}
fn insert_aligned (aligned : u32 , val : u32 , shift : u32 , mask : u32) -> u32 { (aligned & ! (mask << shift)) | ((val & mask) << shift) }
};
}
