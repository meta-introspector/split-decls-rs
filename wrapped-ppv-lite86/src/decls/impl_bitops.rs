macro_rules! deps {
    () => {
        Swap64!();
        AndNot!();
    };
}

macro_rules! impl_bitops {
    () => {
        deps!();
        macro_rules ! impl_bitops { ($ vec : ident) => { impl Not for $ vec { type Output = Self ; # [inline (always)] fn not (self) -> Self :: Output { omap (self , | x | ! x) } } impl BitAnd for $ vec { type Output = Self ; # [inline (always)] fn bitand (self , rhs : Self) -> Self :: Output { omap2 (self , rhs , | x , y | x & y) } } impl BitOr for $ vec { type Output = Self ; # [inline (always)] fn bitor (self , rhs : Self) -> Self :: Output { omap2 (self , rhs , | x , y | x | y) } } impl BitXor for $ vec { type Output = Self ; # [inline (always)] fn bitxor (self , rhs : Self) -> Self :: Output { omap2 (self , rhs , | x , y | x ^ y) } } impl AndNot for $ vec { type Output = Self ; # [inline (always)] fn andnot (self , rhs : Self) -> Self :: Output { omap2 (self , rhs , | x , y | ! x & y) } } impl BitAndAssign for $ vec { # [inline (always)] fn bitand_assign (& mut self , rhs : Self) { * self = * self & rhs } } impl BitOrAssign for $ vec { # [inline (always)] fn bitor_assign (& mut self , rhs : Self) { * self = * self | rhs } } impl BitXorAssign for $ vec { # [inline (always)] fn bitxor_assign (& mut self , rhs : Self) { * self = * self ^ rhs } } impl Swap64 for $ vec { # [inline (always)] fn swap1 (self) -> Self { qmap (self , | x | { ((x & 0x5555555555555555) << 1) | ((x & 0xaaaaaaaaaaaaaaaa) >> 1) }) } # [inline (always)] fn swap2 (self) -> Self { qmap (self , | x | { ((x & 0x3333333333333333) << 2) | ((x & 0xcccccccccccccccc) >> 2) }) } # [inline (always)] fn swap4 (self) -> Self { qmap (self , | x | { ((x & 0x0f0f0f0f0f0f0f0f) << 4) | ((x & 0xf0f0f0f0f0f0f0f0) >> 4) }) } # [inline (always)] fn swap8 (self) -> Self { qmap (self , | x | { ((x & 0x00ff00ff00ff00ff) << 8) | ((x & 0xff00ff00ff00ff00) >> 8) }) } # [inline (always)] fn swap16 (self) -> Self { dmap (self , | x | x . rotate_left (16)) } # [inline (always)] fn swap32 (self) -> Self { qmap (self , | x | x . rotate_left (32)) } # [inline (always)] fn swap64 (self) -> Self { omap (self , | x | (x << 64) | (x >> 64)) } } } ; }
    };
}

impl_bitops!()