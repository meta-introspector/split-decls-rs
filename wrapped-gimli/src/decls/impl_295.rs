macro_rules! deps {
    () => {
        Reader!();
        Result!();
        Error!();
        ReaderOffsetId!();
        Endianity!();
        EndianSlice!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl < 'input , Endian > Reader for EndianSlice < 'input , Endian > where Endian : Endianity , { type Endian = Endian ; type Offset = usize ; # [inline] fn endian (& self) -> Endian { self . endian } # [inline] fn len (& self) -> usize { self . slice . len () } # [inline] fn is_empty (& self) -> bool { self . slice . is_empty () } # [inline] fn empty (& mut self) { self . slice = & [] ; } # [inline] fn truncate (& mut self , len : usize) -> Result < () > { if self . slice . len () < len { Err (Error :: UnexpectedEof (self . offset_id ())) } else { self . slice = & self . slice [.. len] ; Ok (()) } } # [inline] fn offset_from (& self , base : & Self) -> usize { self . offset_from (* base) } # [inline] fn offset_id (& self) -> ReaderOffsetId { ReaderOffsetId (self . slice . as_ptr () as u64) } # [inline] fn lookup_offset_id (& self , id : ReaderOffsetId) -> Option < Self :: Offset > { let id = id . 0 ; let self_id = self . slice . as_ptr () as u64 ; let self_len = self . slice . len () as u64 ; if id >= self_id && id <= self_id + self_len { Some ((id - self_id) as usize) } else { None } } # [inline] fn find (& self , byte : u8) -> Result < usize > { self . find (byte) . ok_or_else (| | Error :: UnexpectedEof (self . offset_id ())) } # [inline] fn skip (& mut self , len : usize) -> Result < () > { if self . slice . len () < len { Err (Error :: UnexpectedEof (self . offset_id ())) } else { self . slice = & self . slice [len ..] ; Ok (()) } } # [inline] fn split (& mut self , len : usize) -> Result < Self > { let slice = self . read_slice (len) ? ; Ok (EndianSlice :: new (slice , self . endian)) } # [cfg (not (feature = "read"))] fn cannot_implement () -> super :: reader :: seal_if_no_alloc :: Sealed { super :: reader :: seal_if_no_alloc :: Sealed } # [cfg (feature = "read")] # [inline] fn to_slice (& self) -> Result < Cow < '_ , [u8] > > { Ok (self . slice . into ()) } # [cfg (feature = "read")] # [inline] fn to_string (& self) -> Result < Cow < '_ , str > > { match str :: from_utf8 (self . slice) { Ok (s) => Ok (s . into ()) , _ => Err (Error :: BadUtf8) , } } # [cfg (feature = "read")] # [inline] fn to_string_lossy (& self) -> Result < Cow < '_ , str > > { Ok (String :: from_utf8_lossy (self . slice)) } # [inline] fn read_slice (& mut self , buf : & mut [u8]) -> Result < () > { let slice = self . read_slice (buf . len ()) ? ; buf . copy_from_slice (slice) ; Ok (()) } }
    };
}

impl_295!()