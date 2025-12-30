// Generated macro for InputConverter (struct)
macro_rules! Depcrate_parser_combinatorInputConverter {
() => {
// Module: crate::parser::combinator
// Provides: {"InputConverter"}
// Dependencies: {}
pub struct InputConverter < InputInner , P , C > where InputInner : Stream , { pub parser : P , pub converter : C , pub _marker : PhantomData < fn (InputInner) > , }
};
}
