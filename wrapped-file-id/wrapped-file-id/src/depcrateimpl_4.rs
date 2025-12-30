// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl FileId { pub fn new_inode (device_id : u64 , inode_number : u64) -> Self { FileId :: Inode { device_id , inode_number , } } pub fn new_low_res (volume_serial_number : u32 , file_index : u64) -> Self { FileId :: LowRes { volume_serial_number , file_index , } } pub fn new_high_res (volume_serial_number : u64 , file_id : u128) -> Self { FileId :: HighRes { volume_serial_number , file_id , } } }
};
}
