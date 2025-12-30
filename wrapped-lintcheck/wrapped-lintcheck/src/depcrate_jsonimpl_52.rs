// Generated macro for impl_52 (impl)
macro_rules! Depcrate_jsonimpl_52 {
() => {
// Module: crate::json
// Provides: {"impl_52"}
// Dependencies: {}
impl fmt :: Display for Summary { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("\
| Lint | Added | Removed | Changed |
| ---- | ----: | ------: | ------: |
" ,) ? ; for SummaryRow { name , added , changed , removed , } in & self . 0 { let html_id = to_html_id (name) ; writeln ! (f , "| [`{name}`](#{html_id}) | {added} | {removed} | {changed} |") ? ; } Ok (()) } }
};
}
