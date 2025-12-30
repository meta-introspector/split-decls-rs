// Generated macro for impl_47 (impl)
macro_rules! Depcrate_writerimpl_47 {
() => {
// Module: crate::writer
// Provides: {"impl_47"}
// Dependencies: {}
impl fmt :: Debug for Writer { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Writer") . field ("state" , & self . state) . field ("delimiter" , & self . delimiter) . field ("term" , & self . term) . field ("style" , & self . style) . field ("quote" , & self . quote) . field ("escape" , & self . escape) . field ("double_quote" , & self . double_quote) . finish () } }
};
}
