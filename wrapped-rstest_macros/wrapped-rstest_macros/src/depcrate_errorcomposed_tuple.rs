// Generated macro for composed_tuple (macro)
macro_rules! Depcrate_errorcomposed_tuple {
() => {
// Module: crate::error
// Provides: {"composed_tuple"}
// Dependencies: {}
macro_rules ! composed_tuple { ($ i : ident) => { $ i } ; ($ i : ident , $ ($ is : ident) , +) => { ($ i , composed_tuple ! ($ ($ is) ,*)) } ; }
};
}
