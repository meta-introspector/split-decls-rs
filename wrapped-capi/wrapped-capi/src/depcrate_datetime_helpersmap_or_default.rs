// Generated macro for map_or_default (function)
macro_rules! Depcrate_datetime_helpersmap_or_default {
() => {
// Module: crate::datetime_helpers
// Provides: {"map_or_default"}
// Dependencies: {}
pub (crate) fn map_or_default < Input , Output > (input : Option < Input >) -> Output where Output : From < Input > + Default , { input . map (Output :: from) . unwrap_or_default () }
};
}
