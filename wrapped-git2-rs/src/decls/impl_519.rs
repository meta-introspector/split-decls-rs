macro_rules! deps {
    () => {
        Oid!();
        Error!();
        ObjectType!();
    };
}

macro_rules! impl_519 {
    () => {
        deps!();
        impl Oid { # [doc = " Parse a hex-formatted object id into an Oid structure."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns an error if the string is empty, is longer than 40 hex"] # [doc = " characters, or contains any non-hex characters."] pub fn from_str (s : & str) -> Result < Oid , Error > { crate :: init () ; let mut raw = raw :: git_oid { id : [0 ; raw :: GIT_OID_RAWSZ] , } ; unsafe { try_call ! (raw :: git_oid_fromstrn (& mut raw , s . as_bytes () . as_ptr () as * const libc :: c_char , s . len () as libc :: size_t)) ; } Ok (Oid { raw }) } # [doc = " Parse a raw object id into an Oid structure."] # [doc = ""] # [doc = " If the array given is not 20 bytes in length, an error is returned."] pub fn from_bytes (bytes : & [u8]) -> Result < Oid , Error > { crate :: init () ; let mut raw = raw :: git_oid { id : [0 ; raw :: GIT_OID_RAWSZ] , } ; if bytes . len () != raw :: GIT_OID_RAWSZ { Err (Error :: from_str ("raw byte array must be 20 bytes")) } else { unsafe { try_call ! (raw :: git_oid_fromraw (& mut raw , bytes . as_ptr ())) ; } Ok (Oid { raw }) } } # [doc = " Creates an all zero Oid structure."] pub fn zero () -> Oid { let out = raw :: git_oid { id : [0 ; raw :: GIT_OID_RAWSZ] , } ; Oid { raw : out } } # [doc = " Hashes the provided data as an object of the provided type, and returns"] # [doc = " an Oid corresponding to the result. This does not store the object"] # [doc = " inside any object database or repository."] pub fn hash_object (kind : ObjectType , bytes : & [u8]) -> Result < Oid , Error > { crate :: init () ; let mut out = raw :: git_oid { id : [0 ; raw :: GIT_OID_RAWSZ] , } ; unsafe { try_call ! (raw :: git_odb_hash (& mut out , bytes . as_ptr () as * const libc :: c_void , bytes . len () , kind . raw ())) ; } Ok (Oid { raw : out }) } # [doc = " Hashes the content of the provided file as an object of the provided type,"] # [doc = " and returns an Oid corresponding to the result. This does not store the object"] # [doc = " inside any object database or repository."] pub fn hash_file < P : AsRef < Path > > (kind : ObjectType , path : P) -> Result < Oid , Error > { crate :: init () ; let rpath = path . as_ref () . into_c_string () ? ; let mut out = raw :: git_oid { id : [0 ; raw :: GIT_OID_RAWSZ] , } ; unsafe { try_call ! (raw :: git_odb_hashfile (& mut out , rpath , kind . raw ())) ; } Ok (Oid { raw : out }) } # [doc = " View this OID as a byte-slice 20 bytes in length."] pub fn as_bytes (& self) -> & [u8] { & self . raw . id } # [doc = " Test if this OID is all zeros."] pub fn is_zero (& self) -> bool { unsafe { raw :: git_oid_iszero (& self . raw) == 1 } } }
    };
}

impl_519!();