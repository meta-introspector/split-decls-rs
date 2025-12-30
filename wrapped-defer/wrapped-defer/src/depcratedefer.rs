// Generated macro for defer (macro)
macro_rules! Depcratedefer {
() => {
// Module: crate
// Provides: {"defer"}
// Dependencies: {}
# [doc = " Defer execution of a closure until the current scope end."] # [macro_export] macro_rules ! defer { ($ e : expr) => { let _defer = $ crate :: defer (|| $ e) ; } ; }
};
}
