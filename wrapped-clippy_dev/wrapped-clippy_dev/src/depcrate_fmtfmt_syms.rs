// Generated macro for fmt_syms (function)
macro_rules! Depcrate_fmtfmt_syms {
() => {
// Module: crate::fmt
// Provides: {"fmt_syms"}
// Dependencies: {}
# [doc = " Format the symbols list"] fn fmt_syms (update_mode : UpdateMode) { FileUpdater :: default () . update_file_checked ("cargo dev fmt" , update_mode , "clippy_utils/src/sym.rs" , & mut | _ , text : & str , new_text : & mut String | { let (pre , conf) = text . split_once ("generate! {\n") . expect ("can't find generate! call") ; let (conf , post) = conf . split_once ("\n}\n") . expect ("can't find end of generate! call") ; let mut lines = conf . lines () . map (| line | { let line = line . trim () ; line . strip_suffix (',') . unwrap_or (line) . trim_end () }) . collect :: < Vec < _ > > () ; lines . sort_unstable () ; write ! (new_text , "{pre}generate! {{\n    {},\n}}\n{post}" , lines . join (",\n    ") ,) . unwrap () ; if text == new_text { UpdateStatus :: Unchanged } else { UpdateStatus :: Changed } } ,) ; }
};
}
