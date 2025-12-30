// Generated macro for impl_819 (impl)
macro_rules! Depcrate_read_macho_segmentimpl_819 {
() => {
// Module: crate::read::macho::segment
// Provides: {"impl_819"}
// Dependencies: {}
impl < 'data , 'file , Mach , R > MachOSegment < 'data , 'file , Mach , R > where Mach : MachHeader , R : ReadRef < 'data > , { # [doc = " Get the Mach-O file containing this segment."] pub fn macho_file (& self) -> & 'file MachOFile < 'data , Mach , R > { self . file } # [doc = " Get the raw Mach-O segment structure."] pub fn macho_segment (& self) -> & 'data Mach :: Segment { self . internal . segment } fn bytes (& self) -> Result < & 'data [u8] > { self . internal . segment . data (self . file . endian , self . internal . data) . read_error ("Invalid Mach-O segment size or offset") } }
};
}
