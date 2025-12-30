// Generated macro for impl_590 (impl)
macro_rules! Depcrate_parser_regeximpl_590 {
() => {
// Module: crate::parser::regex
// Provides: {"impl_590"}
// Dependencies: {}
impl < 'a , Input , R > Parser < Input > for Find < R , Input > where R : Regex < Input :: Range > , Input : RangeStream , Input :: Range : crate :: stream :: Range , { type Output = Input :: Range ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { let (end , First (value)) = self . 0 . find_iter (input . range ()) ; match value { Some (value) => take (end) . parse_lazy (input) . map (| _ | value) , None => PeekErr (Input :: Error :: empty (input . position ()) . into ()) , } } fn add_error (& mut self , error : & mut Tracked < < Input as StreamOnce > :: Error >) { error . error . add (StreamError :: expected_format (format_args ! ("/{}/" , self . 0 . as_str ()))) } }
};
}
