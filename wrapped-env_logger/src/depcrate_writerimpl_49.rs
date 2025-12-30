// Generated macro for impl_49 (impl)
macro_rules! Depcrate_writerimpl_49 {
() => {
// Module: crate::writer
// Provides: {"impl_49"}
// Dependencies: {}
# [cfg (feature = "color")] impl From < anstream :: ColorChoice > for WriteStyle { fn from (choice : anstream :: ColorChoice) -> Self { match choice { anstream :: ColorChoice :: Auto => Self :: Auto , anstream :: ColorChoice :: Always => Self :: Always , anstream :: ColorChoice :: AlwaysAnsi => Self :: Always , anstream :: ColorChoice :: Never => Self :: Never , } } }
};
}
