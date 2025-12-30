// Generated macro for impl_203 (impl)
macro_rules! Depcrate_logoimpl_203 {
() => {
// Module: crate::logo
// Provides: {"impl_203"}
// Dependencies: {}
impl Size { const fn as_str (self) -> & 'static str { match self { Self :: Tiny => Self :: tiny () , Self :: Small => Self :: small () , } } const fn tiny () -> & 'static str { indoc ! { "
            ▛▚▗▀▖▜▘▞▚▝▛▐ ▌▌
            ▛▚▐▀▌▐ ▛▜ ▌▝▄▘▌
        " } } const fn small () -> & 'static str { indoc ! { "
            █▀▀▄ ▄▀▀▄▝▜▛▘▄▀▀▄▝▜▛▘█  █ █
            █▀▀▄ █▀▀█ ▐▌ █▀▀█ ▐▌ ▀▄▄▀ █
        " } } }
};
}
