// Generated macro for expensive_static (macro)
macro_rules! Depcrateexpensive_static {
() => {
// Module: crate
// Provides: {"expensive_static"}
// Dependencies: {}
macro_rules ! expensive_static { ($ name : ident : $ T : ty = $ e : expr ; $ count : tt) => { pub static $ name : $ T = const_repeat ! ($ count $ e , $ T) ; } ; }
};
}
