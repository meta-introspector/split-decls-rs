// Generated macro for is_arm (function)
macro_rules! Depcrateis_arm {
() => {
// Module: crate
// Provides: {"is_arm"}
// Dependencies: {}
fn is_arm (target : & TargetInfo < '_ >) -> bool { matches ! (target . arch , "aarch64" | "arm64ec" | "arm") }
};
}
