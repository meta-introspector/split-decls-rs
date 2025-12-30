// Generated macro for impl_84 (impl)
macro_rules! Depcrateimpl_84 {
() => {
// Module: crate
// Provides: {"impl_84"}
// Dependencies: {}
impl fmt :: Display for CountError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CountError :: OutOfBounds => f . write_str ("${count} out of bounds") , CountError :: Misplaced => f . write_str ("${count} misplaced") , } } }
};
}
