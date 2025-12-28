macro_rules! deps {
    () => {
        Sha3_256!();
        UnknownCryptoError!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        # [cfg_attr (docsrs , doc (cfg (feature = "safe_api")))] # [doc = " Example: hashing from a [`Read`](std::io::Read)er with SHA3-256."] # [doc = " ```rust"] # [doc = " use orion::{"] # [doc = "     hazardous::hash::sha3::sha3_256::{Sha3_256, Digest},"] # [doc = "     errors::UnknownCryptoError,"] # [doc = " };"] # [doc = " use std::io::{self, Read, Write};"] # [doc = ""] # [doc = " // `reader` could also be a `File::open(...)?`."] # [doc = " let mut reader = io::Cursor::new(b\"some data\");"] # [doc = " let mut hasher = Sha3_256::new();"] # [doc = " std::io::copy(&mut reader, &mut hasher)?;"] # [doc = ""] # [doc = " let digest: Digest = hasher.finalize()?;"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [cfg (feature = "safe_api")] impl io :: Write for Sha3_256 { # [doc = " Update the hasher's internal state with *all* of the bytes given."] # [doc = " If this function returns the `Ok` variant, it's guaranteed that it"] # [doc = " will contain the length of the buffer passed to [`Write`](std::io::Write)."] # [doc = " Note that this function is just a small wrapper over"] # [doc = " [`Sha3_256::update`](crate::hazardous::hash::sha3::sha3_256::Sha3_256::update)."] # [doc = ""] # [doc = " ## Errors:"] # [doc = " This function will only ever return the [`std::io::ErrorKind::Other`]()"] # [doc = " variant when it returns an error. Additionally, this will always contain Orion's"] # [doc = " [`UnknownCryptoError`] type."] fn write (& mut self , bytes : & [u8]) -> io :: Result < usize > { self . update (bytes) . map_err (io :: Error :: other) ? ; Ok (bytes . len ()) } # [doc = " This type doesn't buffer writes, so flushing is a no-op."] fn flush (& mut self) -> Result < () , io :: Error > { Ok (()) } }
    };
}

impl_174!()