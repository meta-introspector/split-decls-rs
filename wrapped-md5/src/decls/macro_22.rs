macro_rules! deps {
    () => {
        Md5Core!();
    };
}

macro_rules! macro_22 {
    () => {
        deps!();
        digest :: buffer_fixed ! (# [doc = " MD5 hasher state."] pub struct Md5 (block_api :: Md5Core) ; oid : "1.2.840.113549.2.5" ; impl : FixedHashTraits ;) ;
    };
}

macro_22!();