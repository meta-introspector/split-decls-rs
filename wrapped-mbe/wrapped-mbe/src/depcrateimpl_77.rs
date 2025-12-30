// Generated macro for impl_77 (impl)
macro_rules! Depcrateimpl_77 {
() => {
// Module: crate
// Provides: {"impl_77"}
// Dependencies: {}
impl fmt :: Display for ParseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ParseError :: UnexpectedToken (it) => f . write_str (it) , ParseError :: Expected (it) => f . write_str (it) , ParseError :: InvalidRepeat => f . write_str ("invalid repeat") , ParseError :: RepetitionEmptyTokenTree => f . write_str ("empty token tree in repetition") , } } }
};
}
