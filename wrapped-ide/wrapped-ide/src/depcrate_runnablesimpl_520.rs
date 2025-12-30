// Generated macro for impl_520 (impl)
macro_rules! Depcrate_runnablesimpl_520 {
() => {
// Module: crate::runnables
// Provides: {"impl_520"}
// Dependencies: {}
impl Runnable { pub fn label (& self , target : Option < & str >) -> String { match & self . kind { RunnableKind :: Test { test_id , .. } => format ! ("test {test_id}") , RunnableKind :: TestMod { path } => format ! ("test-mod {path}") , RunnableKind :: Bench { test_id } => format ! ("bench {test_id}") , RunnableKind :: DocTest { test_id , .. } => format ! ("doctest {test_id}") , RunnableKind :: Bin => { format ! ("run {}" , target . unwrap_or ("binary")) } } } pub fn title (& self) -> String { let mut s = String :: from ("▶\u{fe0e} Run ") ; if self . use_name_in_title { format_to ! (s , "{}" , self . nav . name) ; if ! matches ! (self . kind , RunnableKind :: Bin) { s . push (' ') ; } } let suffix = match & self . kind { RunnableKind :: TestMod { .. } => "Tests" , RunnableKind :: Test { .. } => "Test" , RunnableKind :: DocTest { .. } => "Doctest" , RunnableKind :: Bench { .. } => "Bench" , RunnableKind :: Bin => return s , } ; s . push_str (suffix) ; s } }
};
}
