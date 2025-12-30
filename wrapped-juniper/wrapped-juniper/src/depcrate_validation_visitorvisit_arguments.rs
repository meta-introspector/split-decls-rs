// Generated macro for visit_arguments (function)
macro_rules! Depcrate_validation_visitorvisit_arguments {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_arguments"}
// Dependencies: {}
fn visit_arguments < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , meta_args : Option < & 'a Vec < Argument < S > > > , arguments : & 'a Option < Spanning < Arguments < S > > > ,) where S : ScalarValue , V : Visitor < 'a , S > , { if let Some (ref arguments) = * arguments { for argument in arguments . item . iter () { let arg_type = meta_args . and_then (| args | args . iter () . find (| a | a . name == argument . 0 . item)) . map (| a | & a . arg_type) ; ctx . with_pushed_input_type (arg_type , | ctx | { v . enter_argument (ctx , argument) ; visit_input_value (v , ctx , & argument . 1) ; v . exit_argument (ctx , argument) ; }) } } }
};
}
