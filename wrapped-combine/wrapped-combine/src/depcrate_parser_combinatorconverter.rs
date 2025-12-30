// Generated macro for Converter (trait)
macro_rules! Depcrate_parser_combinatorConverter {
() => {
// Module: crate::parser::combinator
// Provides: {"Converter"}
// Dependencies: {}
pub trait Converter < 'a , Input > where Input : Stream , { type InputInner : Stream + 'a ; fn convert (& mut self , input : & 'a mut Input) -> Result < Self :: InputInner , Input :: Error > ; fn convert_error (& mut self , input : & 'a mut Input , error : < Self :: InputInner as StreamOnce > :: Error ,) -> Input :: Error ; }
};
}
