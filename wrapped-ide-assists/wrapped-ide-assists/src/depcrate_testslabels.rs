// Generated macro for labels (function)
macro_rules! Depcrate_testslabels {
() => {
// Module: crate::tests
// Provides: {"labels"}
// Dependencies: {}
fn labels (assists : & [Assist]) -> String { let mut labels = assists . iter () . map (| assist | { let mut label = match & assist . group { Some (g) => g . 0 . clone () , None => assist . label . to_string () , } ; label . push ('\n') ; label }) . collect :: < Vec < _ > > () ; labels . dedup () ; labels . into_iter () . collect :: < String > () }
};
}
