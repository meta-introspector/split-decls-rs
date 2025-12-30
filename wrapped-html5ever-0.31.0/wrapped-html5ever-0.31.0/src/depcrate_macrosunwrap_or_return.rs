// Generated macro for unwrap_or_return (macro)
macro_rules! Depcrate_macrosunwrap_or_return {
() => {
// Module: crate::macros
// Provides: {"unwrap_or_return"}
// Dependencies: {}
macro_rules ! unwrap_or_return { ($ opt : expr) => { unwrap_or_else ! ($ opt , { return ; }) } ; ($ opt : expr , $ retval : expr) => { unwrap_or_else ! ($ opt , { return $ retval }) } ; }
};
}
