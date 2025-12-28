macro_rules! deps {
    () => {
        File!();
        Error!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        # [doc = " Checksums and verify checksums"] impl File { # [doc = " The checksum in the trailer of this pack data file"] pub fn checksum (& self) -> gix_hash :: ObjectId { gix_hash :: ObjectId :: from_bytes_or_panic (& self . data [self . data . len () - self . hash_len ..]) } # [doc = " Verifies that the checksum of the packfile over all bytes preceding it indeed matches the actual checksum,"] # [doc = " returning the actual checksum equivalent to the return value of [`checksum()`][File::checksum()] if there"] # [doc = " is no mismatch."] # [doc = ""] # [doc = " Note that if no `progress` is desired, one can pass [`gix_features::progress::Discard`]."] # [doc = ""] # [doc = " Have a look at [`index::File::verify_integrity(…)`][crate::index::File::verify_integrity()] for an"] # [doc = " even more thorough integrity check."] pub fn verify_checksum (& self , progress : & mut dyn Progress , should_interrupt : & AtomicBool ,) -> Result < gix_hash :: ObjectId , checksum :: Error > { crate :: verify :: checksum_on_disk_or_mmap (self . path () , & self . data , self . checksum () , self . object_hash , progress , should_interrupt ,) } }
    };
}

impl_83!()