// Generated macro for backend (module)
macro_rules! Depcratebackend {
() => {
// Module: crate
// Provides: {"backend"}
// Dependencies: {}
# [doc = " Re-exports for the backend implementations."] pub mod backend { pub use ratatui_core :: backend :: { Backend , ClearType , TestBackend , WindowSize } ; # [cfg (feature = "crossterm")] pub use ratatui_crossterm :: { CrosstermBackend , FromCrossterm , IntoCrossterm } ; # [cfg (all (not (windows) , feature = "termion"))] pub use ratatui_termion :: { FromTermion , IntoTermion , TermionBackend } ; # [cfg (feature = "termwiz")] pub use ratatui_termwiz :: { FromTermwiz , IntoTermwiz , TermwizBackend } ; }
};
}
