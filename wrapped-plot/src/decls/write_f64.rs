macro_rules! write_f64 {
    () => {
        fn write_f64 (w : & mut impl std :: io :: Write , f : f64) -> std :: io :: Result < () > { w . write_all (& f . to_bits () . to_le_bytes ()) }
    };
}

write_f64!();