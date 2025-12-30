// Generated macro for impl_98 (impl)
macro_rules! Depcrate_errorimpl_98 {
() => {
// Module: crate::error
// Provides: {"impl_98"}
// Dependencies: {}
impl TemplateError { # [allow (deprecated)] pub fn of (e : TemplateErrorReason) -> TemplateError { TemplateError { reason : Box :: new (e) , template_name : None , line_no : None , column_no : None , segment : None , } } pub fn at (mut self , template_str : & str , line_no : usize , column_no : usize) -> TemplateError { self . line_no = Some (line_no) ; self . column_no = Some (column_no) ; self . segment = Some (template_segment (template_str , line_no , column_no)) ; self } pub fn in_template (mut self , name : String) -> TemplateError { self . template_name = Some (name) ; self } # [doc = " Get underlying reason for the error"] pub fn reason (& self) -> & TemplateErrorReason { & self . reason } # [doc = " Get the line number and column number of this error"] pub fn pos (& self) -> Option < (usize , usize) > { match (self . line_no , self . column_no) { (Some (line_no) , Some (column_no)) => Some ((line_no , column_no)) , _ => None , } } # [doc = " Get template name of this error"] # [doc = " Returns `None` when the template has no associated name"] pub fn name (& self) -> Option < & String > { self . template_name . as_ref () } }
};
}
