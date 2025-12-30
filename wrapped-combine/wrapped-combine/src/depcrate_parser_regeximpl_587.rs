// Generated macro for impl_587 (impl)
macro_rules! Depcrate_parser_regeximpl_587 {
() => {
// Module: crate::parser::regex
// Provides: {"impl_587"}
// Dependencies: {}
impl < 'a , Input , R > Parser < Input > for Match < R , Input > where R : Regex < Input :: Range > , Input : RangeStream , { type Output = Input :: Range ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { if self . 0 . is_match (input . range ()) { PeekOk (input . range ()) } else { PeekErr (Input :: Error :: empty (input . position ()) . into ()) } } fn add_error (& mut self , error : & mut Tracked < < Input as StreamOnce > :: Error >) { error . error . add (StreamError :: expected_format (format_args ! ("/{}/" , self . 0 . as_str ()))) } }
};
}
