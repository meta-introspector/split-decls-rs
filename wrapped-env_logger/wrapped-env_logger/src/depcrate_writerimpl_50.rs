// Generated macro for impl_50 (impl)
macro_rules! Depcrate_writerimpl_50 {
() => {
// Module: crate::writer
// Provides: {"impl_50"}
// Dependencies: {}
# [cfg (feature = "color")] impl From < WriteStyle > for anstream :: ColorChoice { fn from (choice : WriteStyle) -> Self { match choice { WriteStyle :: Auto => anstream :: ColorChoice :: Auto , WriteStyle :: Always => anstream :: ColorChoice :: Always , WriteStyle :: Never => anstream :: ColorChoice :: Never , } } }
};
}
