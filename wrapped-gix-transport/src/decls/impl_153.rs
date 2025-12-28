macro_rules! deps {
    () => {
        Capability!();
        Capabilities!();
        Protocol!();
        Error!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl Capabilities { # [doc = " Parse capabilities from the given `bytes`."] # [doc = ""] # [doc = " Useful in case they are encoded within a `ref` behind a null byte."] pub fn from_bytes (bytes : & [u8]) -> Result < (Capabilities , usize) , Error > { let delimiter_pos = bytes . find_byte (0) . ok_or (Error :: MissingDelimitingNullByte) ? ; if delimiter_pos + 1 == bytes . len () { return Err (Error :: NoCapabilities) ; } let capabilities = & bytes [delimiter_pos + 1 ..] ; Ok ((Capabilities { data : capabilities . as_bstr () . to_owned () , value_sep : b' ' , } , delimiter_pos ,)) } # [doc = " Parse capabilities from the given a `lines_buf` which is expected to be all newline separated lines"] # [doc = " from the server."] # [doc = ""] # [doc = " Useful for parsing capabilities from a data sent from a server, and to avoid having to deal with"] # [doc = " blocking and async traits for as long as possible. There is no value in parsing a few bytes"] # [doc = " in a non-blocking fashion."] pub fn from_lines (lines_buf : BString) -> Result < Capabilities , Error > { let mut lines = < _ as bstr :: ByteSlice > :: lines (lines_buf . as_slice () . trim ()) ; let version_line = lines . next () . ok_or (Error :: MissingVersionLine) ? ; let (name , value) = version_line . split_at (version_line . find (b" ") . ok_or_else (| | Error :: MalformattedVersionLine (version_line . to_owned () . into ())) ? ,) ; if name != b"version" { return Err (Error :: MalformattedVersionLine (version_line . to_owned () . into ())) ; } if value != b" 2" { return Err (Error :: UnsupportedVersion { desired : Protocol :: V2 , actual : value . to_owned () . into () , }) ; } Ok (Capabilities { value_sep : b'\n' , data : lines . as_bytes () . into () , }) } # [doc = " Returns true of the given `feature` is mentioned in this list of capabilities."] pub fn contains (& self , feature : & str) -> bool { self . capability (feature) . is_some () } # [doc = " Returns the capability with `name`."] pub fn capability (& self , name : & str) -> Option < Capability < '_ > > { self . iter () . find (| c | c . name () == name . as_bytes () . as_bstr ()) } # [doc = " Returns an iterator over all capabilities."] pub fn iter (& self) -> impl Iterator < Item = Capability < '_ > > { self . data . split (move | b | * b == self . value_sep) . map (| c | Capability (c . as_bstr ())) } }
    };
}

impl_153!()