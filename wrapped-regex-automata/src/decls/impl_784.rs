macro_rules! deps {
    () => {
        Start!();
        SerializeError!();
        DeserializeError!();
        StartByteMap!();
        LookMatcher!();
    };
}

macro_rules! impl_784 {
    () => {
        deps!();
        impl StartByteMap { # [doc = " Create a new map from byte values to their corresponding starting"] # [doc = " configurations. The map is determined, in part, by how look-around"] # [doc = " assertions are matched via the matcher given."] pub (crate) fn new (lookm : & LookMatcher) -> StartByteMap { let mut map = [Start :: NonWordByte ; 256] ; map [usize :: from (b'\n')] = Start :: LineLF ; map [usize :: from (b'\r')] = Start :: LineCR ; map [usize :: from (b'_')] = Start :: WordByte ; let mut byte = b'0' ; while byte <= b'9' { map [usize :: from (byte)] = Start :: WordByte ; byte += 1 ; } byte = b'A' ; while byte <= b'Z' { map [usize :: from (byte)] = Start :: WordByte ; byte += 1 ; } byte = b'a' ; while byte <= b'z' { map [usize :: from (byte)] = Start :: WordByte ; byte += 1 ; } let lineterm = lookm . get_line_terminator () ; if lineterm != b'\r' && lineterm != b'\n' { map [usize :: from (lineterm)] = Start :: CustomLineTerminator ; } StartByteMap { map } } # [doc = " Return the starting configuration for the given look-behind byte."] # [doc = ""] # [doc = " If no look-behind exists, callers should use `Start::Text`."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn get (& self , byte : u8) -> Start { self . map [usize :: from (byte)] } # [doc = " Deserializes a byte class map from the given slice. If the slice is of"] # [doc = " insufficient length or otherwise contains an impossible mapping, then"] # [doc = " an error is returned. Upon success, the number of bytes read along with"] # [doc = " the map are returned. The number of bytes read is always a multiple of"] # [doc = " 8."] pub (crate) fn from_bytes (slice : & [u8] ,) -> Result < (StartByteMap , usize) , DeserializeError > { wire :: check_slice_len (slice , 256 , "start byte map") ? ; let mut map = [Start :: NonWordByte ; 256] ; for (i , & repr) in slice [.. 256] . iter () . enumerate () { map [i] = match Start :: from_usize (usize :: from (repr)) { Some (start) => start , None => { return Err (DeserializeError :: generic ("found invalid starting configuration" ,)) } } ; } Ok ((StartByteMap { map } , 256)) } # [doc = " Writes this map to the given byte buffer. if the given buffer is too"] # [doc = " small, then an error is returned. Upon success, the total number of"] # [doc = " bytes written is returned. The number of bytes written is guaranteed to"] # [doc = " be a multiple of 8."] pub (crate) fn write_to (& self , dst : & mut [u8] ,) -> Result < usize , SerializeError > { let nwrite = self . write_to_len () ; if dst . len () < nwrite { return Err (SerializeError :: buffer_too_small ("start byte map")) ; } for (i , & start) in self . map . iter () . enumerate () { dst [i] = start . as_u8 () ; } Ok (nwrite) } # [doc = " Returns the total number of bytes written by `write_to`."] pub (crate) fn write_to_len (& self) -> usize { 256 } }
    };
}

impl_784!()