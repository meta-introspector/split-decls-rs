// Generated macro for rustup (function)
macro_rules! Depcraterustup {
() => {
// Module: crate
// Provides: {"rustup"}
// Dependencies: {}
fn rustup < 'a > (args : impl IntoIterator < Item = & 'a str > , env : Option < (& str , & str) >) -> Command { let mut cmd = Command :: new ("rustup") ; cmd . args (args) . env ("RUSTUP_TOOLCHAIN" , "") ; if let Some ((name , val)) = env { cmd . env (name , val) ; } cmd }
};
}
