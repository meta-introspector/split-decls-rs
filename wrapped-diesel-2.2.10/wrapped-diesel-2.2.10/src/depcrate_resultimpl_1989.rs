// Generated macro for impl_1989 (impl)
macro_rules! Depcrate_resultimpl_1989 {
() => {
// Module: crate::result
// Provides: {"impl_1989"}
// Dependencies: {}
impl DeserializeFieldError { # [cold] pub (crate) fn new < 'a , F , DB > (field : F , error : Box < dyn std :: error :: Error + Send + Sync >) -> Self where DB : crate :: backend :: Backend , F : crate :: row :: Field < 'a , DB > , { DeserializeFieldError { field_name : field . field_name () . map (| s | s . to_string ()) , error , } } }
};
}
