// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl Parse for ProcessMatchInput { fn parse (input : ParseStream) -> Result < Self > { let _file_token = input . parse :: < Ident > () ? ; let _colon1 = input . parse :: < Token ! [:] > () ? ; let file = input . parse :: < LitStr > () ? ; let _comma1 = input . parse :: < Token ! [,] > () ? ; let _line_token = input . parse :: < Ident > () ? ; let _colon2 = input . parse :: < Token ! [:] > () ? ; let line = input . parse :: < syn :: LitInt > () ? ; let _comma2 = input . parse :: < Token ! [,] > () ? ; let _column_token = input . parse :: < Ident > () ? ; let _colon3 = input . parse :: < Token ! [:] > () ? ; let column = input . parse :: < syn :: LitInt > () ? ; let _comma3 = input . parse :: < Token ! [,] > () ? ; let _text_token = input . parse :: < Ident > () ? ; let _colon4 = input . parse :: < Token ! [:] > () ? ; let text = input . parse :: < LitStr > () ? ; Ok (ProcessMatchInput { _file_token , _colon1 , file , _comma1 , _line_token , _colon2 , line , _comma2 , _column_token , _colon3 , column , _comma3 , _text_token , _colon4 , text , }) } }
};
}
