// Generated macro for impl_104 (impl)
macro_rules! Depcrate_section_helpimpl_104 {
() => {
// Module: crate::section::help
// Provides: {"impl_104"}
// Dependencies: {}
impl Display for HelpInfo { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { HelpInfo :: Note (note , theme) => { write ! (f , "{}: {}" , "Note" . style (theme . help_info_note) , note) } HelpInfo :: Warning (warning , theme) => write ! (f , "{}: {}" , "Warning" . style (theme . help_info_warning) , warning) , HelpInfo :: Suggestion (suggestion , theme) => write ! (f , "{}: {}" , "Suggestion" . style (theme . help_info_suggestion) , suggestion) , HelpInfo :: Custom (section) => write ! (f , "{section}") , HelpInfo :: Error (error , theme) => { let errors = std :: iter :: successors (Some (error . as_ref () as & (dyn std :: error :: Error + 'static)) , | e | e . source () ,) ; write ! (f , "Error:") ? ; for (n , error) in errors . enumerate () { writeln ! (f) ? ; write ! (indented (f) . ind (n) , "{}" , error . style (theme . help_info_error)) ? ; } Ok (()) } } } }
};
}
