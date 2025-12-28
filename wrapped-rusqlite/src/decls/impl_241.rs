macro_rules! deps {
    () => {
        Connection!();
        OwnedData!();
        Error!();
        Result!();
        SharedData!();
        Name!();
        Data!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl Connection { # [doc = " Serialize a database."] pub fn serialize < N : Name > (& self , schema : N) -> Result < Data < '_ > > { let schema = schema . as_cstr () ? ; let mut sz = 0 ; let mut ptr : * mut u8 = unsafe { ffi :: sqlite3_serialize (self . handle () , schema . as_ptr () , & mut sz , ffi :: SQLITE_SERIALIZE_NOCOPY ,) } ; Ok (if ptr . is_null () { ptr = unsafe { ffi :: sqlite3_serialize (self . handle () , schema . as_ptr () , & mut sz , 0) } ; if ptr . is_null () { return Err (unsafe { error_from_handle (self . handle () , ffi :: SQLITE_NOMEM) }) ; } Data :: Owned (OwnedData { ptr : NonNull :: new (ptr) . unwrap () , sz : sz . try_into () . unwrap () , }) } else { Data :: Shared (SharedData { ptr : NonNull :: new (ptr) . unwrap () , sz : sz . try_into () . unwrap () , phantom : PhantomData , }) }) } # [doc = " Deserialize from stream"] pub fn deserialize_read_exact < N : Name , R : std :: io :: Read > (& mut self , schema : N , mut read : R , sz : usize , read_only : bool ,) -> Result < () > { let ptr = unsafe { ffi :: sqlite3_malloc64 (sz . try_into () . unwrap ()) } . cast :: < u8 > () ; if ptr . is_null () { return Err (error_from_sqlite_code (ffi :: SQLITE_NOMEM , None)) ; } let buf = unsafe { std :: slice :: from_raw_parts_mut (ptr , sz) } ; read . read_exact (buf) . map_err (| e | { Error :: SqliteFailure (ffi :: Error { code : ffi :: ErrorCode :: CannotOpen , extended_code : ffi :: SQLITE_IOERR , } , Some (format ! ("{e}")) ,) }) ? ; let ptr = NonNull :: new (ptr) . unwrap () ; let data = unsafe { OwnedData :: from_raw_nonnull (ptr , sz) } ; self . deserialize (schema , data , read_only) } # [doc = " Deserialize `include_bytes` as a read only database"] pub fn deserialize_bytes < N : Name > (& mut self , schema : N , data : & 'static [u8]) -> Result < () > { let sz = data . len () . try_into () . unwrap () ; self . deserialize_ (schema , data . as_ptr () as * mut _ , sz , ffi :: SQLITE_DESERIALIZE_READONLY ,) } # [doc = " Deserialize a database."] pub fn deserialize < N : Name > (& mut self , schema : N , data : OwnedData , read_only : bool ,) -> Result < () > { let (data , sz) = data . into_raw () ; let sz = sz . try_into () . unwrap () ; let flags = if read_only { ffi :: SQLITE_DESERIALIZE_FREEONCLOSE | ffi :: SQLITE_DESERIALIZE_READONLY } else { ffi :: SQLITE_DESERIALIZE_FREEONCLOSE | ffi :: SQLITE_DESERIALIZE_RESIZEABLE } ; self . deserialize_ (schema , data , sz , flags) } fn deserialize_ < N : Name > (& mut self , schema : N , data : * mut u8 , sz : ffi :: sqlite_int64 , flags : std :: ffi :: c_uint ,) -> Result < () > { let schema = schema . as_cstr () ? ; let rc = unsafe { ffi :: sqlite3_deserialize (self . handle () , schema . as_ptr () , data , sz , sz , flags) } ; if rc != ffi :: SQLITE_OK { return Err (unsafe { error_from_handle (self . handle () , rc) }) ; } Ok (()) } }
    };
}

impl_241!()