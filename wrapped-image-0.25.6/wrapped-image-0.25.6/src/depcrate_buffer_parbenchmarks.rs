// Generated macro for benchmarks (module)
macro_rules! Depcrate_buffer_parbenchmarks {
() => {
// Module: crate::buffer_par
// Provides: {"benchmarks"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "benchmarks")] mod benchmarks { use crate :: { Rgb , RgbImage } ; const S : u32 = 1024 ; # [bench] fn creation (b : & mut test :: Bencher) { let mut bytes = 0 ; b . iter (| | { let img = RgbImage :: from_fn (S , S , | _ , _ | test :: black_box (pixel_func ())) ; bytes += img . as_raw () . len () as u64 ; }) ; b . bytes = bytes ; } # [bench] fn creation_par (b : & mut test :: Bencher) { let mut bytes = 0 ; b . iter (| | { let img = RgbImage :: from_par_fn (S , S , | _ , _ | test :: black_box (pixel_func ())) ; bytes += img . as_raw () . len () as u64 ; }) ; b . bytes = bytes ; } fn pixel_func () -> Rgb < u8 > { use std :: collections :: hash_map :: RandomState ; use std :: hash :: { BuildHasher , Hasher } ; Rgb (std :: array :: from_fn (| _ | { RandomState :: new () . build_hasher () . finish () as u8 })) } }
};
}
