macro_rules! newline_count {
    () => {
        fn newline_count (body : & str) -> usize { # [cfg (feature = "simd")] { memchr :: memchr_iter (b'\n' , body . as_bytes ()) . count () } # [cfg (not (feature = "simd"))] { body . lines () . count () . saturating_sub (1) } }
    };
}

newline_count!();