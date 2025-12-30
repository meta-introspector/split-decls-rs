// Generated macro for parse_file (function)
macro_rules! Depcrateparse_file {
() => {
// Module: crate
// Provides: {"parse_file"}
// Dependencies: {}
pub fn parse_file (input : & str , silent : bool) { let source = read_file (input) . expect ("Read file failed") ; let res = parse (source . as_str ()) ; match res { Ok (res) => print_entries_resource (& res) , Err ((res , errors)) => { print_entries_resource (& res) ; if silent { return ; } ; println ! ("==============================\n") ; if errors . len () == 1 { println ! ("Parser encountered one error:") ; } else { println ! ("Parser encountered {} errors:" , errors . len ()) ; } println ! ("-----------------------------") ; for err in errors { println ! ("{:#?}" , err) ; if let Some (slice) = err . slice { let (id , desc) = get_error_info (err . kind) ; let end_pos = cmp :: min (err . pos . end , slice . end) ; let snippet = Snippet { slices : vec ! [Slice { source : source [slice . clone ()] . to_string () , line_start : get_line_num (& source , err . pos . start) + 1 , origin : Some (input . to_string ()) , fold : false , annotations : vec ! [SourceAnnotation { label : desc . to_string () , annotation_type : AnnotationType :: Error , range : (err . pos . start - slice . start , end_pos - slice . start + 1) , }] , }] , title : Some (Annotation { label : Some (desc . to_string ()) , id : Some (id . to_string ()) , annotation_type : AnnotationType :: Error , }) , footer : vec ! [] , } ; let dl = DisplayList :: from (snippet) ; let dlf = DisplayListFormatter :: new (true , false) ; println ! ("{}" , dlf . format (& dl)) ; println ! ("-----------------------------") ; } } } } ; }
};
}
