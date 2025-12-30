// Generated macro for strip_windows_collation_suffix_lossy (function)
macro_rules! Depcrate_locale_windowsstrip_windows_collation_suffix_lossy {
() => {
// Module: crate::locale::windows
// Provides: {"strip_windows_collation_suffix_lossy"}
// Dependencies: {}
fn strip_windows_collation_suffix_lossy (lcid : & str) -> (& str , Option < Value >) { if let Some ((prefix , suffix)) = lcid . split_once ('_') { let collation_value = match suffix { "phoneb" => value ! ("phonebk") , "pronun" => value ! ("zhuyin") , "radstr" => value ! ("unihan") , "stroke" => value ! ("stroke") , "tradnl" => value ! ("trad") , _ => return (prefix , None) , } ; (prefix , Some (collation_value)) } else { (lcid , None) } }
};
}
