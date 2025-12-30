// Generated macro for impl_sh_ops (macro)
macro_rules! Depcrate_doubleimpl_sh_ops {
() => {
// Module: crate::double
// Provides: {"impl_sh_ops"}
// Dependencies: {}
macro_rules ! impl_sh_ops { ($ t : ty) => { impl Shl <$ t > for udouble { type Output = Self ; # [inline] fn shl (self , rhs : $ t) -> Self :: Output { match rhs { 0 => self , s if s >= umax :: BITS as $ t => Self { hi : self . lo << (s - umax :: BITS as $ t) , lo : 0 , } , s => Self { lo : self . lo << s , hi : (self . hi << s) | (self . lo >> (umax :: BITS as $ t - s)) , } , } } } impl ShlAssign <$ t > for udouble { # [inline] fn shl_assign (& mut self , rhs : $ t) { match rhs { 0 => { } s if s >= umax :: BITS as $ t => { self . hi = self . lo << (s - umax :: BITS as $ t) ; self . lo = 0 ; } s => { self . hi <<= s ; self . hi |= self . lo >> (umax :: BITS as $ t - s) ; self . lo <<= s ; } } } } impl Shr <$ t > for udouble { type Output = Self ; # [inline] fn shr (self , rhs : $ t) -> Self :: Output { match rhs { 0 => self , s if s >= umax :: BITS as $ t => Self { lo : self . hi >> (rhs - umax :: BITS as $ t) , hi : 0 , } , s => Self { hi : self . hi >> s , lo : (self . lo >> s) | (self . hi << (umax :: BITS as $ t - s)) , } , } } } impl ShrAssign <$ t > for udouble { # [inline] fn shr_assign (& mut self , rhs : $ t) { match rhs { 0 => { } s if s >= umax :: BITS as $ t => { self . lo = self . hi >> (rhs - umax :: BITS as $ t) ; self . hi = 0 ; } s => { self . lo >>= s ; self . lo |= self . hi << (umax :: BITS as $ t - s) ; self . hi >>= s ; } } } } } ; }
};
}
