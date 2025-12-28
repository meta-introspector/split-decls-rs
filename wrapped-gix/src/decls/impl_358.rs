macro_rules! deps {
    () => {
        Error!();
        Note!();
        Read!();
        Repository!();
        Id!();
        Blob!();
        Kind!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        # [doc = " Write objects of any type."] impl crate :: Repository { # [doc = " Write the given object into the object database and return its object id."] # [doc = ""] # [doc = " Note that we hash the object in memory to avoid storing objects that are already present. That way,"] # [doc = " we avoid writing duplicate objects using slow disks that will eventually have to be garbage collected."] pub fn write_object (& self , object : impl gix_object :: WriteTo) -> Result < Id < '_ > , object :: write :: Error > { let mut buf = self . empty_reusable_buffer () ; object . write_to (buf . deref_mut ()) . map_err (| err | Box :: new (err) as Box < dyn std :: error :: Error + Send + Sync + 'static >) ? ; self . write_object_inner (& buf , object . kind ()) } fn write_object_inner (& self , buf : & [u8] , kind : gix_object :: Kind) -> Result < Id < '_ > , object :: write :: Error > { let oid = gix_object :: compute_hash (self . object_hash () , kind , buf) . map_err (| err | Box :: new (err) as Box < dyn std :: error :: Error + Send + Sync >) ? ; if self . objects . exists (& oid) { return Ok (oid . attach (self)) ; } self . objects . write_buf (kind , buf) . map (| oid | oid . attach (self)) . map_err (Into :: into) } # [doc = " Write a blob from the given `bytes`."] # [doc = ""] # [doc = " We avoid writing duplicate objects to slow disks that will eventually have to be garbage collected by"] # [doc = " pre-hashing the data, and checking if the object is already present."] pub fn write_blob (& self , bytes : impl AsRef < [u8] >) -> Result < Id < '_ > , object :: write :: Error > { let bytes = bytes . as_ref () ; let oid = gix_object :: compute_hash (self . object_hash () , gix_object :: Kind :: Blob , bytes) . map_err (| err | Box :: new (err) as Box < dyn std :: error :: Error + Send + Sync >) ? ; if self . objects . exists (& oid) { return Ok (oid . attach (self)) ; } self . objects . write_buf (gix_object :: Kind :: Blob , bytes) . map_err (Into :: into) . map (| oid | oid . attach (self)) } # [doc = " Write a blob from the given `Read` implementation."] # [doc = ""] # [doc = " Note that we hash the object in memory to avoid storing objects that are already present. That way,"] # [doc = " we avoid writing duplicate objects using slow disks that will eventually have to be garbage collected."] # [doc = ""] # [doc = " If that is prohibitive, use the object database directly."] pub fn write_blob_stream (& self , mut bytes : impl std :: io :: Read) -> Result < Id < '_ > , object :: write :: Error > { let mut buf = self . empty_reusable_buffer () ; std :: io :: copy (& mut bytes , buf . deref_mut ()) . map_err (| err | Box :: new (err) as Box < dyn std :: error :: Error + Send + Sync >) ? ; self . write_blob_stream_inner (& buf) } fn write_blob_stream_inner (& self , buf : & [u8]) -> Result < Id < '_ > , object :: write :: Error > { let oid = gix_object :: compute_hash (self . object_hash () , gix_object :: Kind :: Blob , buf) . map_err (| err | Box :: new (err) as Box < dyn std :: error :: Error + Send + Sync >) ? ; if self . objects . exists (& oid) { return Ok (oid . attach (self)) ; } self . objects . write_buf (gix_object :: Kind :: Blob , buf) . map_err (Into :: into) . map (| oid | oid . attach (self)) } }
    };
}

impl_358!();