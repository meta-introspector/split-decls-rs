#[macro_export]
macro_rules! MacroExpanderNew {
    ($cx:expr, $monotonic:expr) => {
        MacroExpander { cx: $cx, monotonic: $monotonic }
    };
}