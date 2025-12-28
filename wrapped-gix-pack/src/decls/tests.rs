macro_rules! tests {
    () => {
        # [cfg (test)] mod tests { use super :: * ; # [test] fn leb64_encode_max_int () { let mut buf = [0u8 ; 10] ; let buf = leb64_encode (u64 :: MAX , & mut buf) ; assert_eq ! (buf . len () , 10 , "10 bytes should be used when 64bits are encoded") ; } }
    };
}

tests!();