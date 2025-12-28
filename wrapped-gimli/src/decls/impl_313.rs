macro_rules! deps {
    () => {
        EndianReader!();
        Result!();
        Reader!();
        Error!();
        ReaderOffsetId!();
        Endianity!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl < Endian , T > Reader for EndianReader < Endian , T > where Endian : Endianity , T : CloneStableDeref < Target = [u8] > + Debug , { type Endian = Endian ; type Offset = usize ; # [inline] fn endian (& self) -> Endian { self . endian } # [inline] fn len (& self) -> usize { self . range . len () } # [inline] fn empty (& mut self) { self . range . truncate (0) ; } # [inline] fn truncate (& mut self , len : usize) -> Result < () > { if self . len () < len { Err (Error :: UnexpectedEof (self . offset_id ())) } else { self . range . truncate (len) ; Ok (()) } } # [inline] fn offset_from (& self , base : & EndianReader < Endian , T >) -> usize { let base_ptr = base . bytes () . as_ptr () as usize ; let ptr = self . bytes () . as_ptr () as usize ; debug_assert ! (base_ptr <= ptr) ; debug_assert ! (ptr + self . bytes () . len () <= base_ptr + base . bytes () . len ()) ; ptr - base_ptr } # [inline] fn offset_id (& self) -> ReaderOffsetId { ReaderOffsetId (self . bytes () . as_ptr () as u64) } # [inline] fn lookup_offset_id (& self , id : ReaderOffsetId) -> Option < Self :: Offset > { let id = id . 0 ; let self_id = self . bytes () . as_ptr () as u64 ; let self_len = self . bytes () . len () as u64 ; if id >= self_id && id <= self_id + self_len { Some ((id - self_id) as usize) } else { None } } # [inline] fn find (& self , byte : u8) -> Result < usize > { self . bytes () . iter () . position (| x | * x == byte) . ok_or_else (| | Error :: UnexpectedEof (self . offset_id ())) } # [inline] fn skip (& mut self , len : usize) -> Result < () > { if self . len () < len { Err (Error :: UnexpectedEof (self . offset_id ())) } else { self . range . skip (len) ; Ok (()) } } # [inline] fn split (& mut self , len : usize) -> Result < Self > { if self . len () < len { Err (Error :: UnexpectedEof (self . offset_id ())) } else { let mut r = self . clone () ; r . range . truncate (len) ; self . range . skip (len) ; Ok (r) } } # [inline] fn to_slice (& self) -> Result < Cow < '_ , [u8] > > { Ok (self . bytes () . into ()) } # [inline] fn to_string (& self) -> Result < Cow < '_ , str > > { match str :: from_utf8 (self . bytes ()) { Ok (s) => Ok (s . into ()) , _ => Err (Error :: BadUtf8) , } } # [inline] fn to_string_lossy (& self) -> Result < Cow < '_ , str > > { Ok (String :: from_utf8_lossy (self . bytes ())) } # [inline] fn read_slice (& mut self , buf : & mut [u8]) -> Result < () > { match self . range . read_slice (buf . len ()) { Some (slice) => { buf . copy_from_slice (slice) ; Ok (()) } None => Err (Error :: UnexpectedEof (self . offset_id ())) , } } }
    };
}

impl_313!();