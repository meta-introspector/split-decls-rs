// Generated macro for impl_477 (impl)
macro_rules! Depcrate_reportimpl_477 {
() => {
// Module: crate::report
// Provides: {"impl_477"}
// Dependencies: {}
impl fmt :: Debug for BenchmarkId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fn format_opt (opt : & Option < String >) -> String { match * opt { Some (ref string) => format ! ("\"{}\"" , string) , None => "None" . to_owned () , } } write ! (f , "BenchmarkId {{ group_id: \"{}\", function_id: {}, value_str: {}, throughput: {:?} }}" , self . group_id , format_opt (& self . function_id) , format_opt (& self . value_str) , self . throughput ,) } }
};
}
