// Generated macro for looks_like_rustdoc (function)
macro_rules! Depcratelooks_like_rustdoc {
() => {
// Module: crate
// Provides: {"looks_like_rustdoc"}
// Dependencies: {}
# [doc = " Returns `true` if our flags look like they may be for rustdoc, i.e., this is cargo calling us to"] # [doc = " be rustdoc. It's hard to be sure as cargo does not have a RUSTDOC_WRAPPER or an env var that"] # [doc = " would let us get a clear signal."] fn looks_like_rustdoc () -> bool { env :: args () . any (| arg | arg == "--test-run-directory") }
};
}
