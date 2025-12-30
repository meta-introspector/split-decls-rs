// Generated macro for input_converter (function)
macro_rules! Depcrate_parser_combinatorinput_converter {
() => {
// Module: crate::parser::combinator
// Provides: {"input_converter"}
// Dependencies: {}
pub fn input_converter < Input , InputInner , P , C > (parser : P , converter : C ,) -> InputConverter < InputInner , P , C > where Input : Stream , InputInner : Stream , P : Parser < InputInner > , for < 'c > C : Converter < 'c , Input , InputInner = InputInner > , { InputConverter { parser , converter , _marker : PhantomData , } }
};
}
