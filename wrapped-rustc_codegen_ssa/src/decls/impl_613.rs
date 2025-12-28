macro_rules! deps {
    () => {
        CodegenResults!();
        CodegenErrors!();
    };
}

macro_rules! impl_613 {
    () => {
        deps!();
        impl CodegenResults { pub fn serialize_rlink (sess : & Session , rlink_file : & Path , codegen_results : & CodegenResults , metadata : & EncodedMetadata , outputs : & OutputFilenames ,) -> Result < usize , io :: Error > { let mut encoder = FileEncoder :: new (rlink_file) ? ; encoder . emit_raw_bytes (RLINK_MAGIC) ; encoder . emit_raw_bytes (& RLINK_VERSION . to_be_bytes ()) ; encoder . emit_str (sess . cfg_version) ; Encodable :: encode (codegen_results , & mut encoder) ; Encodable :: encode (metadata , & mut encoder) ; Encodable :: encode (outputs , & mut encoder) ; encoder . finish () . map_err (| (_path , err) | err) } pub fn deserialize_rlink (sess : & Session , data : Vec < u8 > ,) -> Result < (Self , EncodedMetadata , OutputFilenames) , CodegenErrors > { if ! data . starts_with (RLINK_MAGIC) { return Err (CodegenErrors :: WrongFileType) ; } let data = & data [RLINK_MAGIC . len () ..] ; if data . len () < 4 { return Err (CodegenErrors :: EmptyVersionNumber) ; } let mut version_array : [u8 ; 4] = Default :: default () ; version_array . copy_from_slice (& data [.. 4]) ; if u32 :: from_be_bytes (version_array) != RLINK_VERSION { return Err (CodegenErrors :: EncodingVersionMismatch { version_array : String :: from_utf8_lossy (& version_array) . to_string () , rlink_version : RLINK_VERSION , }) ; } let Ok (mut decoder) = MemDecoder :: new (& data [4 ..] , 0) else { return Err (CodegenErrors :: CorruptFile) ; } ; let rustc_version = decoder . read_str () ; if rustc_version != sess . cfg_version { return Err (CodegenErrors :: RustcVersionMismatch { rustc_version : rustc_version . to_string () , }) ; } let codegen_results = CodegenResults :: decode (& mut decoder) ; let metadata = EncodedMetadata :: decode (& mut decoder) ; let outputs = OutputFilenames :: decode (& mut decoder) ; Ok ((codegen_results , metadata , outputs)) } }
    };
}

impl_613!();