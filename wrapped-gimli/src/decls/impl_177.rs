macro_rules! deps {
    () => {
        Endianity!();
        EndianSlice!();
        EhFrame!();
    };
}

macro_rules! impl_177 {
    () => {
        deps!();
        impl < 'input , Endian > EhFrame < EndianSlice < 'input , Endian > > where Endian : Endianity , { # [doc = " Construct a new `EhFrame` instance from the data in the"] # [doc = " `.eh_frame` section."] # [doc = ""] # [doc = " It is the caller's responsibility to read the section and present it as"] # [doc = " a `&[u8]` slice. That means using some ELF loader on Linux, a Mach-O"] # [doc = " loader on macOS, etc."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{EhFrame, EndianSlice, NativeEndian};"] # [doc = ""] # [doc = " // Use with `.eh_frame`"] # [doc = " # let buf = [0x00, 0x01, 0x02, 0x03];"] # [doc = " # let read_eh_frame_section_somehow = || &buf;"] # [doc = " let eh_frame = EhFrame::new(read_eh_frame_section_somehow(), NativeEndian);"] # [doc = " ```"] pub fn new (section : & 'input [u8] , endian : Endian) -> Self { Self :: from (EndianSlice :: new (section , endian)) } }
    };
}

impl_177!()