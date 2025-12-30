// Generated macro for gen_buffer_unstable_docs (macro)
macro_rules! Depcrate_constructorsgen_buffer_unstable_docs {
() => {
// Module: crate::constructors
// Provides: {"gen_buffer_unstable_docs"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! gen_buffer_unstable_docs { (BUFFER , $ data : path) => { concat ! ("A version of [`" , stringify ! ($ data) , "`] that uses custom data " , "provided by a [`BufferProvider`].\n\n" , "✨ *Enabled with the `serde` feature.*\n\n" , "[📚 Help choosing a constructor](icu_provider::constructors)" ,) } ; (UNSTABLE , $ data : path) => { concat ! ("A version of [`" , stringify ! ($ data) , "`] that uses custom data " , "provided by a [`DataProvider`].\n\n" , "[📚 Help choosing a constructor](icu_provider::constructors)\n\n" , "<div class=\"stab unstable\">⚠️ The bounds on <tt>provider</tt> may change over time, including in SemVer minor releases.</div>") } ; }
};
}
