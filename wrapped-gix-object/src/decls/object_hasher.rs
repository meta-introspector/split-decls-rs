macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! object_hasher {
    () => {
        deps!();
        fn object_hasher (hash_kind : gix_hash :: Kind , object_kind : Kind , object_size : u64) -> gix_hash :: Hasher { let mut hasher = gix_hash :: hasher (hash_kind) ; hasher . update (& encode :: loose_header (object_kind , object_size)) ; hasher }
    };
}

object_hasher!();