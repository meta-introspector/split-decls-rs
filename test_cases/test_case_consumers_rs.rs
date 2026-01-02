// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/consumers.rs
// Error: expected square brackets
// Problematic line: line 15

pub use super::dataflow::{BorrowIndex, Borrows, calculate_borrows_out_of_scope_at_location};
pub use super::place_ext::PlaceExt;
pub use super::places_conflict::{PlaceConflictBias, places_conflict};
pub use super::polonius::legacy::{
    PoloniusFacts as PoloniusInput, PoloniusLocationTable, PoloniusOutput, PoloniusRegionVid,
    RichLocation, RustcFacts,
};
