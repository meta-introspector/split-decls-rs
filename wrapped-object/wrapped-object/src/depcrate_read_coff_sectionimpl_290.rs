// Generated macro for impl_290 (impl)
macro_rules! Depcrate_read_coff_sectionimpl_290 {
() => {
// Module: crate::read::coff::section
// Provides: {"impl_290"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > , Coff : CoffHeader > ObjectSegment < 'data > for CoffSegment < 'data , 'file , R , Coff > { # [inline] fn address (& self) -> u64 { u64 :: from (self . section . virtual_address . get (LE)) } # [inline] fn size (& self) -> u64 { u64 :: from (self . section . virtual_size . get (LE)) } # [inline] fn align (& self) -> u64 { self . section . coff_alignment () } # [inline] fn file_range (& self) -> (u64 , u64) { let (offset , size) = self . section . coff_file_range () . unwrap_or ((0 , 0)) ; (u64 :: from (offset) , u64 :: from (size)) } fn data (& self) -> Result < & 'data [u8] > { self . bytes () } fn data_range (& self , address : u64 , size : u64) -> Result < Option < & 'data [u8] > > { Ok (read :: util :: data_range (self . bytes () ? , self . address () , address , size ,)) } # [inline] fn name_bytes (& self) -> Result < Option < & [u8] > > { self . section . name (self . file . common . symbols . strings ()) . map (Some) } # [inline] fn name (& self) -> Result < Option < & str > > { let name = self . section . name (self . file . common . symbols . strings ()) ? ; str :: from_utf8 (name) . ok () . read_error ("Non UTF-8 COFF section name") . map (Some) } # [inline] fn flags (& self) -> SegmentFlags { let characteristics = self . section . characteristics . get (LE) ; SegmentFlags :: Coff { characteristics } } }
};
}
