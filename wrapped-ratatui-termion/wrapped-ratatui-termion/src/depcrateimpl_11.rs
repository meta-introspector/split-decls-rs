// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl < W > TermionBackend < W > where W : Write , { # [doc = " Creates a new Termion backend with the given writer."] # [doc = ""] # [doc = " Most applications will use either [`stdout`](std::io::stdout) or"] # [doc = " [`stderr`](std::io::stderr) as writer. See the [FAQ] to determine which one to use."] # [doc = ""] # [doc = " [FAQ]: https://ratatui.rs/faq/#should-i-use-stdout-or-stderr"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " use std::io::stdout;"] # [doc = ""] # [doc = " use ratatui::backend::TermionBackend;"] # [doc = ""] # [doc = " let backend = TermionBackend::new(stdout());"] # [doc = " ```"] pub const fn new (writer : W) -> Self { Self { writer } } # [doc = " Gets the writer."] # [instability :: unstable (feature = "backend-writer" , issue = "https://github.com/ratatui/ratatui/pull/991")] pub const fn writer (& self) -> & W { & self . writer } # [doc = " Gets the writer as a mutable reference."] # [doc = " Note: writing to the writer may cause incorrect output after the write. This is due to the"] # [doc = " way that the Terminal implements diffing Buffers."] # [instability :: unstable (feature = "backend-writer" , issue = "https://github.com/ratatui/ratatui/pull/991")] pub const fn writer_mut (& mut self) -> & mut W { & mut self . writer } }
};
}
