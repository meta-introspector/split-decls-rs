// Generated macro for impl_821 (impl)
macro_rules! Depcrate_read_macho_segmentimpl_821 {
() => {
// Module: crate::read::macho::segment
// Provides: {"impl_821"}
// Dependencies: {}
impl < 'data , 'file , Mach , R > ObjectSegment < 'data > for MachOSegment < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { # [inline] fn address (& self) -> u64 { self . internal . segment . vmaddr (self . file . endian) . into () } # [inline] fn size (& self) -> u64 { self . internal . segment . vmsize (self . file . endian) . into () } # [inline] fn align (& self) -> u64 { 0x1000 } # [inline] fn file_range (& self) -> (u64 , u64) { self . internal . segment . file_range (self . file . endian) } fn data (& self) -> Result < & 'data [u8] > { self . bytes () } fn data_range (& self , address : u64 , size : u64) -> Result < Option < & 'data [u8] > > { Ok (read :: util :: data_range (self . bytes () ? , self . address () , address , size ,)) } # [inline] fn name_bytes (& self) -> Result < Option < & [u8] > > { Ok (Some (self . internal . segment . name ())) } # [inline] fn name (& self) -> Result < Option < & str > > { Ok (Some (str :: from_utf8 (self . internal . segment . name ()) . ok () . read_error ("Non UTF-8 Mach-O segment name") ? ,)) } # [inline] fn flags (& self) -> SegmentFlags { let flags = self . internal . segment . flags (self . file . endian) ; let maxprot = self . internal . segment . maxprot (self . file . endian) ; let initprot = self . internal . segment . initprot (self . file . endian) ; SegmentFlags :: MachO { flags , maxprot , initprot , } } }
};
}
