// Generated macro for print (macro)
macro_rules! Depcrateprint {
() => {
// Module: crate
// Provides: {"print"}
// Dependencies: {}
macro_rules ! print { ($ ($ tt : tt) *) => { let _ = ufmt :: uwrite ! (simio :: Console , $ ($ tt) *) ; } ; }
};
}
