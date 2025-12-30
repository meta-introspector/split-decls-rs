// Generated macro for impl_187 (impl)
macro_rules! Depcrate_read_anyimpl_187 {
() => {
// Module: crate::read::any
// Provides: {"impl_187"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > > ObjectSegment < 'data > for Segment < 'data , 'file , R > { fn address (& self) -> u64 { with_inner ! (self . inner , SegmentInternal , | x | x . address ()) } fn size (& self) -> u64 { with_inner ! (self . inner , SegmentInternal , | x | x . size ()) } fn align (& self) -> u64 { with_inner ! (self . inner , SegmentInternal , | x | x . align ()) } fn file_range (& self) -> (u64 , u64) { with_inner ! (self . inner , SegmentInternal , | x | x . file_range ()) } fn data (& self) -> Result < & 'data [u8] > { with_inner ! (self . inner , SegmentInternal , | x | x . data ()) } fn data_range (& self , address : u64 , size : u64) -> Result < Option < & 'data [u8] > > { with_inner ! (self . inner , SegmentInternal , | x | x . data_range (address , size)) } fn name_bytes (& self) -> Result < Option < & [u8] > > { with_inner ! (self . inner , SegmentInternal , | x | x . name_bytes ()) } fn name (& self) -> Result < Option < & str > > { with_inner ! (self . inner , SegmentInternal , | x | x . name ()) } fn flags (& self) -> SegmentFlags { with_inner ! (self . inner , SegmentInternal , | x | x . flags ()) } }
};
}
