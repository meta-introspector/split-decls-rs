macro_rules! cksum {
    () => {
        # [doc = " Generate a checksum"] pub fn cksum (s : & [u8]) -> String { Sha256 :: new () . update (s) . finish_hex () }
    };
}

cksum!()