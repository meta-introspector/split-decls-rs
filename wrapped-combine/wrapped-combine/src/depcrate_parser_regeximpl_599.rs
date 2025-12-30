// Generated macro for impl_599 (impl)
macro_rules! Depcrate_parser_regeximpl_599 {
() => {
// Module: crate::parser::regex
// Provides: {"impl_599"}
// Dependencies: {}
impl < 'a , Input , F , G , R > Parser < Input > for CapturesMany < F , G , R , Input > where F : FromIterator < Input :: Range > , G : FromIterator < F > , R : Regex < Input :: Range > , Input : RangeStream , Input :: Range : crate :: stream :: Range , { type Output = G ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { let (end , value) = self . 0 . captures (input . range ()) ; take (end) . parse_lazy (input) . map (| _ | value) } fn add_error (& mut self , error : & mut Tracked < < Input as StreamOnce > :: Error >) { error . error . add (StreamError :: expected_format (format_args ! ("/{}/" , self . 0 . as_str ()))) } }
};
}
