// Generated macro for assume (macro)
macro_rules! Depcrateassume {
() => {
// Module: crate
// Provides: {"assume"}
// Dependencies: {}
macro_rules ! assume { ($ e : expr) => { debug_assert ! ($ e) ; if ! ($ e) { core :: hint :: unreachable_unchecked () ; } } ; }
};
}
