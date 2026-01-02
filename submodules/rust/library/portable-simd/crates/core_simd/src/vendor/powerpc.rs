mkuse!{use crate :: simd :: * ;}
mkuse!{# [cfg (target_arch = "powerpc")] use core :: arch :: powerpc :: * ;}
mkuse!{# [cfg (target_arch = "powerpc64")] use core :: arch :: powerpc64 :: * ;}
mkitem!{from_transmute ! { unsafe f64x2 => vector_double }}
mkitem!{from_transmute ! { unsafe i64x2 => vector_signed_long }}
mkitem!{from_transmute ! { unsafe u64x2 => vector_unsigned_long }}