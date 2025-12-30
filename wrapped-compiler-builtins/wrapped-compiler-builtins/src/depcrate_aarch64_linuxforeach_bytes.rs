// Generated macro for foreach_bytes (macro)
macro_rules! Depcrate_aarch64_linuxforeach_bytes {
() => {
// Module: crate::aarch64_linux
// Provides: {"foreach_bytes"}
// Dependencies: {}
# [macro_export] macro_rules ! foreach_bytes { ($ macro : path , $ name : ident) => { foreach_ordering ! ($ macro , 1 , $ { concat (__aarch64_ , $ name , "1") }) ; foreach_ordering ! ($ macro , 2 , $ { concat (__aarch64_ , $ name , "2") }) ; foreach_ordering ! ($ macro , 4 , $ { concat (__aarch64_ , $ name , "4") }) ; foreach_ordering ! ($ macro , 8 , $ { concat (__aarch64_ , $ name , "8") }) ; } ; }
};
}
