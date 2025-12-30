// Generated macro for commutative_op (macro)
macro_rules! Depcratecommutative_op {
() => {
// Module: crate
// Provides: {"commutative_op"}
// Dependencies: {}
macro_rules ! commutative_op { ($ t : ty) => { impl ops :: Add < ByteSize > for $ t { type Output = ByteSize ; # [inline (always)] fn add (self , rhs : ByteSize) -> ByteSize { ByteSize (rhs . 0 + (self as u64)) } } impl ops :: Mul < ByteSize > for $ t { type Output = ByteSize ; # [inline (always)] fn mul (self , rhs : ByteSize) -> ByteSize { ByteSize (rhs . 0 * (self as u64)) } } } ; }
};
}
