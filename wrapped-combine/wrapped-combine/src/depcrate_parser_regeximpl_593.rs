// Generated macro for impl_593 (impl)
macro_rules! Depcrate_parser_regeximpl_593 {
() => {
// Module: crate::parser::regex
// Provides: {"impl_593"}
// Dependencies: {}
impl < 'a , Input , F , R > Parser < Input > for FindMany < F , R , Input > where F : FromIterator < Input :: Range > , R : Regex < Input :: Range > , Input : RangeStream , Input :: Range : crate :: stream :: Range , { type Output = F ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { let (end , value) = self . 0 . find_iter (input . range ()) ; take (end) . parse_lazy (input) . map (| _ | value) } fn add_error (& mut self , error : & mut Tracked < < Input as StreamOnce > :: Error >) { error . error . add (StreamError :: expected_format (format_args ! ("/{}/" , self . 0 . as_str ()))) } }
};
}
