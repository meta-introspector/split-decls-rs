// Generated macro for multiple_output_types_to_stdout (function)
macro_rules! Depcrate_utilmultiple_output_types_to_stdout {
() => {
// Module: crate::util
// Provides: {"multiple_output_types_to_stdout"}
// Dependencies: {}
fn multiple_output_types_to_stdout (output_types : & OutputTypes , single_output_file_is_stdout : bool ,) -> bool { use std :: io :: IsTerminal ; if std :: io :: stdout () . is_terminal () { let named_text_types = output_types . iter () . filter (| (f , o) | f . is_text_output () && * o == & Some (OutFileName :: Stdout)) . count () ; let unnamed_text_types = output_types . iter () . filter (| (f , o) | f . is_text_output () && o . is_none ()) . count () ; named_text_types > 1 || unnamed_text_types > 1 && single_output_file_is_stdout } else { let named_types = output_types . values () . filter (| o | * o == & Some (OutFileName :: Stdout)) . count () ; let unnamed_types = output_types . values () . filter (| o | o . is_none ()) . count () ; named_types > 1 || unnamed_types > 1 && single_output_file_is_stdout } }
};
}
