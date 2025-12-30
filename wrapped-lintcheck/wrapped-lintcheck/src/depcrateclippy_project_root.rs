// Generated macro for clippy_project_root (function)
macro_rules! Depcrateclippy_project_root {
() => {
// Module: crate
// Provides: {"clippy_project_root"}
// Dependencies: {}
# [doc = " Returns the path to the Clippy project directory"] # [must_use] fn clippy_project_root () -> & 'static Path { Path :: new (env ! ("CARGO_MANIFEST_DIR")) . parent () . unwrap () }
};
}
