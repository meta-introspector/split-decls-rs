// Generated macro for TemplateErrorReason (enum)
macro_rules! Depcrate_errorTemplateErrorReason {
() => {
// Module: crate::error
// Provides: {"TemplateErrorReason"}
// Dependencies: {}
# [doc = " Template parsing error"] # [non_exhaustive] # [derive (Debug , Error)] pub enum TemplateErrorReason { # [error ("helper {0:?} was opened, but {1:?} is closing")] MismatchingClosedHelper (String , String) , # [error ("decorator {0:?} was opened, but {1:?} is closing")] MismatchingClosedDecorator (String , String) , # [error ("invalid handlebars syntax: {0}")] InvalidSyntax (String) , # [error ("invalid parameter {0:?}")] InvalidParam (String) , # [error ("nested subexpression is not supported")] NestedSubexpression , # [error ("Template \"{1}\": {0}")] IoError (IOError , String) , # [cfg (feature = "dir_source")] # [error ("Walk dir error: {err}")] WalkdirError { # [from] err : WalkdirError , } , }
};
}
