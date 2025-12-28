macro_rules! sign_extend {
    () => {
        fn sign_extend (val : u64 , nbytes : usize) -> i64 { let shift = (8 - nbytes) * 8 ; (val << shift) as i64 >> shift }
    };
}

sign_extend!()