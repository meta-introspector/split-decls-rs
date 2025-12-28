macro_rules! byte_to_hash {
    () => {
        fn byte_to_hash (b : u8) -> gix_hash :: Kind { match b { 0 => gix_hash :: Kind :: Sha1 , _ => unreachable ! ("BUG: we control the protocol") , } }
    };
}

byte_to_hash!()