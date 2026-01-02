mkuse!{use crate :: simd :: * ;}
mkuse!{use core :: arch :: wasm32 :: v128 ;}
mkitem!{from_transmute ! { unsafe u8x16 => v128 }}
mkitem!{from_transmute ! { unsafe i8x16 => v128 }}
mkitem!{from_transmute ! { unsafe u16x8 => v128 }}
mkitem!{from_transmute ! { unsafe i16x8 => v128 }}
mkitem!{from_transmute ! { unsafe u32x4 => v128 }}
mkitem!{from_transmute ! { unsafe i32x4 => v128 }}
mkitem!{from_transmute ! { unsafe f32x4 => v128 }}
mkitem!{from_transmute ! { unsafe u64x2 => v128 }}
mkitem!{from_transmute ! { unsafe i64x2 => v128 }}
mkitem!{from_transmute ! { unsafe f64x2 => v128 }}
mkmod!{p32, { 
                getname!(p32);
                getsrc!(p32);
                getpath!(p32);
                get_deps!(p32);
                get_crates!(p32);
                mkinclude!(p32);
                mkuse!{use super :: * ;}
mkitem!{from_transmute ! { unsafe usizex4 => v128 }}
mkitem!{from_transmute ! { unsafe isizex4 => v128 }} 
            }}
mkmod!{p64, { 
                getname!(p64);
                getsrc!(p64);
                getpath!(p64);
                get_deps!(p64);
                get_crates!(p64);
                mkinclude!(p64);
                mkuse!{use super :: * ;}
mkitem!{from_transmute ! { unsafe usizex2 => v128 }}
mkitem!{from_transmute ! { unsafe isizex2 => v128 }} 
            }}