macro_rules! min_u64_usize {
    () => {
        # [inline (always)] # [cfg (feature = "std")] fn min_u64_usize (a : u64 , b : usize) -> usize { match usize :: try_from (a) { Ok (a) => usize :: min (a , b) , Err (_) => b , } }
    };
}

min_u64_usize!();