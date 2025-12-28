macro_rules! deps {
    () => {
        SerialJoin!();
        OutputReader!();
        Hash!();
    };
}

macro_rules! keyed_hash {
    () => {
        deps!();
        # [doc = " The keyed hash function."] # [doc = ""] # [doc = " This is suitable for use as a message authentication code, for example to"] # [doc = " replace an HMAC instance. In that use case, the constant-time equality"] # [doc = " checking provided by [`Hash`](struct.Hash.html) is almost always a security"] # [doc = " requirement, and callers need to be careful not to compare MACs as raw"] # [doc = " bytes."] # [doc = ""] # [doc = " For an incremental version that accepts multiple writes, see [`Hasher::new_keyed`],"] # [doc = " [`Hasher::update`], and [`Hasher::finalize`]. These two lines are equivalent:"] # [doc = ""] # [doc = " ```"] # [doc = " # const KEY: &[u8; 32] = &[0; 32];"] # [doc = " let mac = blake3::keyed_hash(KEY, b\"foo\");"] # [doc = " # let mac1 = mac;"] # [doc = ""] # [doc = " let mac = blake3::Hasher::new_keyed(KEY).update(b\"foo\").finalize();"] # [doc = " # let mac2 = mac;"] # [doc = " # assert_eq!(mac1, mac2);"] # [doc = " ```"] # [doc = ""] # [doc = " For output sizes other than 32 bytes, see [`Hasher::finalize_xof`], and [`OutputReader`]."] # [doc = ""] # [doc = " This function is always single-threaded. For multithreading support, see"] # [doc = " [`Hasher::update_rayon`](struct.Hasher.html#method.update_rayon)."] pub fn keyed_hash (key : & [u8 ; KEY_LEN] , input : & [u8]) -> Hash { let key_words = platform :: words_from_le_bytes_32 (key) ; hash_all_at_once :: < join :: SerialJoin > (input , & key_words , KEYED_HASH) . root_hash () }
    };
}

keyed_hash!();