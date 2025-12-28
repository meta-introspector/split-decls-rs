macro_rules! deps {
    () => {
        Range!();
        Endianity!();
        EndianSlice!();
        LittleEndian!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        # [doc = " # Range Methods"] # [doc = ""] # [doc = " Unfortunately, `std::ops::Index` *must* return a reference, so we can't"] # [doc = " implement `Index<Range<usize>>` to return a new `EndianSlice` the way we would"] # [doc = " like to. Instead, we abandon fancy indexing operators and have these plain"] # [doc = " old methods."] impl < 'input , Endian > EndianSlice < 'input , Endian > where Endian : Endianity , { # [doc = " Take the given `start..end` range of the underlying slice and return a"] # [doc = " new `EndianSlice`."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{EndianSlice, LittleEndian};"] # [doc = ""] # [doc = " let slice = &[0x01, 0x02, 0x03, 0x04];"] # [doc = " let endian_slice = EndianSlice::new(slice, LittleEndian);"] # [doc = " assert_eq!(endian_slice.range(1..3),"] # [doc = "            EndianSlice::new(&slice[1..3], LittleEndian));"] # [doc = " ```"] pub fn range (& self , idx : Range < usize >) -> EndianSlice < 'input , Endian > { EndianSlice { slice : & self . slice [idx] , endian : self . endian , } } # [doc = " Take the given `start..` range of the underlying slice and return a new"] # [doc = " `EndianSlice`."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{EndianSlice, LittleEndian};"] # [doc = ""] # [doc = " let slice = &[0x01, 0x02, 0x03, 0x04];"] # [doc = " let endian_slice = EndianSlice::new(slice, LittleEndian);"] # [doc = " assert_eq!(endian_slice.range_from(2..),"] # [doc = "            EndianSlice::new(&slice[2..], LittleEndian));"] # [doc = " ```"] pub fn range_from (& self , idx : RangeFrom < usize >) -> EndianSlice < 'input , Endian > { EndianSlice { slice : & self . slice [idx] , endian : self . endian , } } # [doc = " Take the given `..end` range of the underlying slice and return a new"] # [doc = " `EndianSlice`."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{EndianSlice, LittleEndian};"] # [doc = ""] # [doc = " let slice = &[0x01, 0x02, 0x03, 0x04];"] # [doc = " let endian_slice = EndianSlice::new(slice, LittleEndian);"] # [doc = " assert_eq!(endian_slice.range_to(..3),"] # [doc = "            EndianSlice::new(&slice[..3], LittleEndian));"] # [doc = " ```"] pub fn range_to (& self , idx : RangeTo < usize >) -> EndianSlice < 'input , Endian > { EndianSlice { slice : & self . slice [idx] , endian : self . endian , } } }
    };
}

impl_286!()