// Generated macro for impl_87 (impl)
macro_rules! Depcrate_errorimpl_87 {
() => {
// Module: crate::error
// Provides: {"impl_87"}
// Dependencies: {}
impl fmt :: Display for RenderError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { let desc = self . reason . to_string () ; match (self . line_no , self . column_no) { (Some (line) , Some (col)) => write ! (f , "Error rendering \"{}\" line {}, col {}: {}" , self . template_name . as_deref () . unwrap_or ("Unnamed template") , line , col , desc) , _ => write ! (f , "{desc}") , } } }
};
}
