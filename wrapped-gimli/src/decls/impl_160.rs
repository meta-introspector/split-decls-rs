macro_rules! deps {
    () => {
        EndianSlice!();
        Endianity!();
        DebugFrame!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < 'input , Endian > DebugFrame < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `DebugFrame` instance from the data in the"] # [doc = " `.debug_frame` section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the section and present it as"] # [doc = " a `&[u8]` slice. That means using some ELF loader on Linux, a Mach-O"] # [doc = " loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{DebugFrame, NativeEndian};"] # [doc = ""] # [doc = " // Use with `.debug_frame`"] # [doc = " # let buf = [0x00, 0x01, 0x02, 0x03];"] # [doc = " # let read_debug_frame_section_somehow = || &buf;"] # [doc = " let debug_frame = DebugFrame::new(read_debug_frame_section_somehow(), NativeEndian);"] # [doc = " ```"] pub fn new (section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (section , endian)) } }
    };
}

impl_160!()