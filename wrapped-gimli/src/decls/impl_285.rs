macro_rules! deps {
    () => {
        EndianSlice!();
        Endianity!();
        Result!();
        Error!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        impl < 'input , Endian > EndianSlice < 'input , Endian > where Endian : Endianity , { # [doc = " Construct a new `EndianSlice` with the given slice and endianity."] # [inline] pub fn new (slice : & 'input [u8] , endian : Endian) -> EndianSlice < 'input , Endian > { EndianSlice { slice , endian } } # [doc = " Return a reference to the raw slice."] # [inline] pub fn slice (& self) -> & 'input [u8] { self . slice } # [doc = " Split the slice in two at the given index, resulting in the tuple where"] # [doc = " the first item has range [0, idx), and the second has range [idx,"] # [doc = " len). Panics if the index is out of bounds."] # [inline] pub fn split_at (& self , idx : usize ,) -> (EndianSlice < 'input , Endian > , EndianSlice < 'input , Endian >) { (self . range_to (.. idx) , self . range_from (idx ..)) } # [doc = " Find the first occurrence of a byte in the slice, and return its index."] # [inline] pub fn find (& self , byte : u8) -> Option < usize > { self . slice . iter () . position (| ch | * ch == byte) } # [doc = " Return the offset of the start of the slice relative to the start"] # [doc = " of the given slice."] # [inline] pub fn offset_from (& self , base : EndianSlice < 'input , Endian >) -> usize { let base_ptr = base . slice . as_ptr () as usize ; let ptr = self . slice . as_ptr () as usize ; debug_assert ! (base_ptr <= ptr) ; debug_assert ! (ptr + self . slice . len () <= base_ptr + base . slice . len ()) ; ptr - base_ptr } # [doc = " Converts the slice to a string using `str::from_utf8`."] # [doc = ""] # [doc = " Returns an error if the slice contains invalid characters."] # [inline] pub fn to_string (& self) -> Result < & 'input str > { str :: from_utf8 (self . slice) . map_err (| _ | Error :: BadUtf8) } # [doc = " Converts the slice to a string, including invalid characters,"] # [doc = " using `String::from_utf8_lossy`."] # [cfg (feature = "read")] # [inline] pub fn to_string_lossy (& self) -> Cow < 'input , str > { String :: from_utf8_lossy (self . slice) } # [inline] fn read_slice (& mut self , len : usize) -> Result < & 'input [u8] > { if self . slice . len () < len { Err (Error :: UnexpectedEof (self . offset_id ())) } else { let val = & self . slice [.. len] ; self . slice = & self . slice [len ..] ; Ok (val) } } }
    };
}

impl_285!()