// Generated macro for impl_102 (impl)
macro_rules! Depcrate_errorimpl_102 {
() => {
// Module: crate::error
// Provides: {"impl_102"}
// Dependencies: {}
impl fmt :: Display for TemplateError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { match (self . line_no , self . column_no , & self . segment) { (Some (line) , Some (col) , Some (seg)) => writeln ! (f , "Template error: {}\n    --> Template error in \"{}\":{}:{}\n     |\n{}     |\n     = reason: {}" , self . reason () , self . template_name . as_ref () . unwrap_or (& "Unnamed template" . to_owned ()) , line , col , seg , self . reason ()) , _ => write ! (f , "{}" , self . reason ()) , } } }
};
}
