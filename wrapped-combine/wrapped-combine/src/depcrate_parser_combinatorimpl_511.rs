// Generated macro for impl_511 (impl)
macro_rules! Depcrate_parser_combinatorimpl_511 {
() => {
// Module: crate::parser::combinator
// Provides: {"impl_511"}
// Dependencies: {}
impl < 'a , Input , InputInner > Converter < 'a , Input > for (fn (& 'a mut Input) -> Result < InputInner , Input :: Error > , fn (& 'a mut Input , InputInner :: Error) -> Input :: Error ,) where Input : Stream , InputInner : Stream + 'a , { type InputInner = InputInner ; fn convert (& mut self , input : & 'a mut Input) -> Result < InputInner , Input :: Error > { (self . 0) (input) } fn convert_error (& mut self , input : & 'a mut Input , error : InputInner :: Error) -> Input :: Error { (self . 1) (input , error) } }
};
}
