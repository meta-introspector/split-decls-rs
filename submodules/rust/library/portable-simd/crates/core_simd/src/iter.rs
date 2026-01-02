mkuse!{use crate :: simd :: { LaneCount , Simd , SupportedLaneCount } ;}
mkuse!{use core :: { iter :: { Product , Sum } , ops :: { Add , Mul } , } ;}
mkitem!{macro_rules ! impl_traits { { $ type : ty } => { impl < const N : usize > Sum < Self > for Simd <$ type , N > where LaneCount < N >: SupportedLaneCount , { # [inline] fn sum < I : Iterator < Item = Self >> (iter : I) -> Self { iter . fold (Simd :: splat (0 as $ type) , Add :: add) } } impl < const N : usize > Product < Self > for Simd <$ type , N > where LaneCount < N >: SupportedLaneCount , { # [inline] fn product < I : Iterator < Item = Self >> (iter : I) -> Self { iter . fold (Simd :: splat (1 as $ type) , Mul :: mul) } } impl <'a , const N : usize > Sum <&'a Self > for Simd <$ type , N > where LaneCount < N >: SupportedLaneCount , { # [inline] fn sum < I : Iterator < Item = &'a Self >> (iter : I) -> Self { iter . fold (Simd :: splat (0 as $ type) , Add :: add) } } impl <'a , const N : usize > Product <&'a Self > for Simd <$ type , N > where LaneCount < N >: SupportedLaneCount , { # [inline] fn product < I : Iterator < Item = &'a Self >> (iter : I) -> Self { iter . fold (Simd :: splat (1 as $ type) , Mul :: mul) } } } }}
mkitem!{impl_traits ! { f32 }}
mkitem!{impl_traits ! { f64 }}
mkitem!{impl_traits ! { u8 }}
mkitem!{impl_traits ! { u16 }}
mkitem!{impl_traits ! { u32 }}
mkitem!{impl_traits ! { u64 }}
mkitem!{impl_traits ! { usize }}
mkitem!{impl_traits ! { i8 }}
mkitem!{impl_traits ! { i16 }}
mkitem!{impl_traits ! { i32 }}
mkitem!{impl_traits ! { i64 }}
mkitem!{impl_traits ! { isize }}