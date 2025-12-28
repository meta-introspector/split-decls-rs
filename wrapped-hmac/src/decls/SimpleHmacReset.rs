macro_rules! SimpleHmacReset {
    () => {
        # [doc = " Simplified HMAC instance with reset support able to operate"] # [doc = " over hash functions which do not expose block-level API and"] # [doc = " hash functions which process blocks lazily (e.g. BLAKE2)."] # [derive (Clone)] pub struct SimpleHmacReset < D : Digest + BlockSizeUser > { digest : D , opad_key : Block < D > , ipad_key : Block < D > , }
    };
}

SimpleHmacReset!()