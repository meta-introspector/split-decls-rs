macro_rules! deps {
    () => {
        SerializeError!();
        Endian!();
        DFA!();
        Flags!();
        DeserializeError!();
        NFA!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Flags { # [doc = " Creates a set of flags for a DFA from an NFA."] # [doc = ""] # [doc = " N.B. This constructor was defined at the time of writing because all"] # [doc = " of the flags are derived directly from the NFA. If this changes in the"] # [doc = " future, we might be more thoughtful about how the `Flags` value is"] # [doc = " itself built."] # [cfg (feature = "dfa-build")] fn from_nfa (nfa : & thompson :: NFA) -> Flags { Flags { has_empty : nfa . has_empty () , is_utf8 : nfa . is_utf8 () , is_always_start_anchored : nfa . is_always_start_anchored () , } } # [doc = " Deserializes the flags from the given slice. On success, this also"] # [doc = " returns the number of bytes read from the slice."] pub (crate) fn from_bytes (slice : & [u8] ,) -> Result < (Flags , usize) , DeserializeError > { let (bits , nread) = wire :: try_read_u32 (slice , "flag bitset") ? ; let flags = Flags { has_empty : bits & (1 << 0) != 0 , is_utf8 : bits & (1 << 1) != 0 , is_always_start_anchored : bits & (1 << 2) != 0 , } ; Ok ((flags , nread)) } # [doc = " Writes these flags to the given byte slice. If the buffer is too small,"] # [doc = " then an error is returned. To determine how big the buffer must be,"] # [doc = " use `write_to_len`."] pub (crate) fn write_to < E : Endian > (& self , dst : & mut [u8] ,) -> Result < usize , SerializeError > { fn bool_to_int (b : bool) -> u32 { if b { 1 } else { 0 } } let nwrite = self . write_to_len () ; if dst . len () < nwrite { return Err (SerializeError :: buffer_too_small ("flag bitset")) ; } let bits = (bool_to_int (self . has_empty) << 0) | (bool_to_int (self . is_utf8) << 1) | (bool_to_int (self . is_always_start_anchored) << 2) ; E :: write_u32 (bits , dst) ; Ok (nwrite) } # [doc = " Returns the number of bytes the serialized form of these flags"] # [doc = " will use."] pub (crate) fn write_to_len (& self) -> usize { size_of :: < u32 > () } }
    };
}

impl_46!()