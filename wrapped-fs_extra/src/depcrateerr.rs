// Generated macro for err (macro)
macro_rules! Depcrateerr {
() => {
// Module: crate
// Provides: {"err"}
// Dependencies: {}
macro_rules ! err { ($ text : expr , $ kind : expr) => { return Err (Error :: new ($ kind , $ text)) } ; ($ text : expr) => { err ! ($ text , ErrorKind :: Other) } ; }
};
}
