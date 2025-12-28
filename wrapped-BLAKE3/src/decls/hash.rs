macro_rules! deps {
    () => {
        OutputReader!();
        Hash!();
    };
}

macro_rules! hash {
    () => {
        deps!();
        # [doc = " The default hash function."] # [doc = ""] # [doc = " For an incremental version that accepts multiple writes, see [`Hasher::new`],"] # [doc = " [`Hasher::update`], and [`Hasher::finalize`]. These two lines are equivalent:"] # [doc = ""] # [doc = " ```"] # [doc = " let hash = blake3::hash(b\"foo\");"] # [doc = " # let hash1 = hash;"] # [doc = ""] # [doc = " let hash = blake3::Hasher::new().update(b\"foo\").finalize();"] # [doc = " # let hash2 = hash;"] # [doc = " # assert_eq!(hash1, hash2);"] # [doc = " ```"] # [doc = ""] # [doc = " For output sizes other than 32 bytes, see [`Hasher::finalize_xof`] and"] # [doc = " [`OutputReader`]."] # [doc = ""] # [doc = " This function is always single-threaded. For multithreading support, see"] # [doc = " [`Hasher::update_rayon`](struct.Hasher.html#method.update_rayon)."] pub fn hash (input : & [u8]) -> Hash { hash_all_at_once :: < join :: SerialJoin > (input , IV , 0) . root_hash () }
    };
}

hash!()