// Generated macro for impl_1086 (impl)
macro_rules! Depcrate_read_wasmimpl_1086 {
() => {
// Module: crate::read::wasm
// Provides: {"impl_1086"}
// Dependencies: {}
impl < 'data , 'file , R > ObjectSegment < 'data > for WasmSegment < 'data , 'file , R > { # [inline] fn address (& self) -> u64 { unreachable ! () } # [inline] fn size (& self) -> u64 { unreachable ! () } # [inline] fn align (& self) -> u64 { unreachable ! () } # [inline] fn file_range (& self) -> (u64 , u64) { unreachable ! () } fn data (& self) -> Result < & 'data [u8] > { unreachable ! () } fn data_range (& self , _address : u64 , _size : u64) -> Result < Option < & 'data [u8] > > { unreachable ! () } # [inline] fn name_bytes (& self) -> Result < Option < & [u8] > > { unreachable ! () } # [inline] fn name (& self) -> Result < Option < & str > > { unreachable ! () } # [inline] fn flags (& self) -> SegmentFlags { unreachable ! () } }
};
}
