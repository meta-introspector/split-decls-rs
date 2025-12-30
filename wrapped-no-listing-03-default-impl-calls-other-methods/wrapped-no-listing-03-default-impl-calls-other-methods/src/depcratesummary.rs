// Generated macro for Summary (trait)
macro_rules! DepcrateSummary {
() => {
// Module: crate
// Provides: {"Summary"}
// Dependencies: {}
pub trait Summary { fn summarize_author (& self) -> String ; fn summarize (& self) -> String { format ! ("(Read more from {}...)" , self . summarize_author ()) } }
};
}
