mkuse!{use crate :: simd :: * ;}
mkuse!{# [cfg (target_arch = "arm")] use core :: arch :: arm :: * ;}
mkuse!{# [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec"))] use core :: arch :: aarch64 :: * ;}
mkmod!{neon, { 
                getname!(neon);
                getsrc!(neon);
                getpath!(neon);
                get_deps!(neon);
                get_crates!(neon);
                mkinclude!(neon);
                mkuse!{use super :: * ;}
mkitem!{from_transmute ! { unsafe f32x2 => float32x2_t }}
mkitem!{from_transmute ! { unsafe f32x4 => float32x4_t }}
mkitem!{from_transmute ! { unsafe u8x8 => uint8x8_t }}
mkitem!{from_transmute ! { unsafe u8x16 => uint8x16_t }}
mkitem!{from_transmute ! { unsafe i8x8 => int8x8_t }}
mkitem!{from_transmute ! { unsafe i8x16 => int8x16_t }}
mkitem!{from_transmute ! { unsafe u8x8 => poly8x8_t }}
mkitem!{from_transmute ! { unsafe u8x16 => poly8x16_t }}
mkitem!{from_transmute ! { unsafe u16x4 => uint16x4_t }}
mkitem!{from_transmute ! { unsafe u16x8 => uint16x8_t }}
mkitem!{from_transmute ! { unsafe i16x4 => int16x4_t }}
mkitem!{from_transmute ! { unsafe i16x8 => int16x8_t }}
mkitem!{from_transmute ! { unsafe u16x4 => poly16x4_t }}
mkitem!{from_transmute ! { unsafe u16x8 => poly16x8_t }}
mkitem!{from_transmute ! { unsafe u32x2 => uint32x2_t }}
mkitem!{from_transmute ! { unsafe u32x4 => uint32x4_t }}
mkitem!{from_transmute ! { unsafe i32x2 => int32x2_t }}
mkitem!{from_transmute ! { unsafe i32x4 => int32x4_t }}
mkitem!{from_transmute ! { unsafe Simd < u64 , 1 > => uint64x1_t }}
mkitem!{from_transmute ! { unsafe u64x2 => uint64x2_t }}
mkitem!{from_transmute ! { unsafe Simd < i64 , 1 > => int64x1_t }}
mkitem!{from_transmute ! { unsafe i64x2 => int64x2_t }}
mkitem!{from_transmute ! { unsafe Simd < u64 , 1 > => poly64x1_t }}
mkitem!{from_transmute ! { unsafe u64x2 => poly64x2_t }} 
            }}
mkmod!{simd32, { 
                getname!(simd32);
                getsrc!(simd32);
                getpath!(simd32);
                get_deps!(simd32);
                get_crates!(simd32);
                mkinclude!(simd32);
                mkuse!{use super :: * ;}
mkitem!{from_transmute ! { unsafe Simd < u8 , 4 > => uint8x4_t }}
mkitem!{from_transmute ! { unsafe Simd < i8 , 4 > => int8x4_t }}
mkitem!{from_transmute ! { unsafe Simd < u16 , 2 > => uint16x2_t }}
mkitem!{from_transmute ! { unsafe Simd < i16 , 2 > => int16x2_t }} 
            }}
mkmod!{aarch64, { 
                getname!(aarch64);
                getsrc!(aarch64);
                getpath!(aarch64);
                get_deps!(aarch64);
                get_crates!(aarch64);
                mkinclude!(aarch64);
                mkuse!{use super :: neon :: * ;}
mkuse!{use super :: * ;}
mkitem!{from_transmute ! { unsafe Simd < f64 , 1 > => float64x1_t }}
mkitem!{from_transmute ! { unsafe f64x2 => float64x2_t }} 
            }}