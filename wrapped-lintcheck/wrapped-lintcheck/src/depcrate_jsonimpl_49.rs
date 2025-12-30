// Generated macro for impl_49 (impl)
macro_rules! Depcrate_jsonimpl_49 {
() => {
// Module: crate::json
// Provides: {"impl_49"}
// Dependencies: {}
impl LintJson { fn key (& self) -> impl Ord + '_ { (self . name . as_str () , self . file_line . as_str ()) } # [doc = " Formats the warning information with an action verb for display."] fn info_text (& self , action : & str) -> String { format ! ("{action} `{}` at [`{}`]({})" , self . name , self . file_line , self . file_url) } }
};
}
