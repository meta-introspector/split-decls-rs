// Generated macro for impl_596 (impl)
macro_rules! Depcrate_parser_regeximpl_596 {
() => {
// Module: crate::parser::regex
// Provides: {"impl_596"}
// Dependencies: {}
impl < 'a , Input , F , R > Parser < Input > for Captures < F , R , Input > where F : FromIterator < Input :: Range > , R : Regex < Input :: Range > , Input : RangeStream , Input :: Range : crate :: stream :: Range , { type Output = F ; type PartialState = () ; # [inline] fn parse_lazy (& mut self , input : & mut Input ,) -> ParseResult < Self :: Output , < Input as StreamOnce > :: Error > { let (end , First (value)) = self . 0 . captures (input . range ()) ; match value { Some (value) => take (end) . parse_lazy (input) . map (| _ | value) , None => PeekErr (Input :: Error :: empty (input . position ()) . into ()) , } } fn add_error (& mut self , error : & mut Tracked < < Input as StreamOnce > :: Error >) { error . error . add (StreamError :: expected_format (format_args ! ("/{}/" , self . 0 . as_str ()))) } }
};
}
