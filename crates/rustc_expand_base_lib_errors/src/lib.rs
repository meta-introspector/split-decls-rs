// Re-export the diagnostic types from their respective files
mod trace_macro_base;
mod trace_macro_note;

pub use trace_macro_base::*;
pub use trace_macro_note::*;