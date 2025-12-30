// Generated macro for TemplateError (struct)
macro_rules! Depcrate_errorTemplateError {
() => {
// Module: crate::error
// Provides: {"TemplateError"}
// Dependencies: {}
# [doc = " Error on parsing template."] # [derive (Debug , Error)] pub struct TemplateError { reason : Box < TemplateErrorReason > , template_name : Option < String > , line_no : Option < usize > , column_no : Option < usize > , segment : Option < String > , }
};
}
