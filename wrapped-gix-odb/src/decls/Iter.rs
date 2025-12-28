macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " The type for an iterator over `Result<gix_hash::ObjectId, Error>)`"] pub struct Iter { inner : fs :: walkdir :: DirEntryIter , hash_hex_len : usize , }
    };
}

Iter!();