// Generated macro for possible_options (function)
macro_rules! Depcrate_renderpossible_options {
() => {
// Module: crate::render
// Provides: {"possible_options"}
// Dependencies: {}
fn possible_options (roff : & mut Roff , arg : & Arg , arg_help_written : bool) { if let Some ((possible_values_text , with_help)) = get_possible_values (arg) { if arg_help_written { roff . text ([Inline :: LineBreak]) ; } if with_help { roff . text ([Inline :: LineBreak , italic ("Possible values:")]) ; roff . control ("RS" , ["14"]) ; for line in possible_values_text { roff . control ("IP" , ["\\(bu" , "2"]) ; roff . text ([roman (line)]) ; } roff . control ("RE" , []) ; } else { let possible_value_text : Vec < Inline > = vec ! [Inline :: LineBreak , roman ("[") , italic ("possible values: ") , roman (possible_values_text . join (", ")) , roman ("]") ,] ; roff . text (possible_value_text) ; } } }
};
}
