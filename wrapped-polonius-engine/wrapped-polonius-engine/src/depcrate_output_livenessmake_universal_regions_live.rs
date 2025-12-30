// Generated macro for make_universal_regions_live (function)
macro_rules! Depcrate_output_livenessmake_universal_regions_live {
() => {
// Module: crate::output::liveness
// Provides: {"make_universal_regions_live"}
// Dependencies: {}
pub (super) fn make_universal_regions_live < T : FactTypes > (origin_live_on_entry : & mut Vec < (T :: Origin , T :: Point) > , cfg_node : & BTreeSet < T :: Point > , universal_regions : & [T :: Origin] ,) { debug ! ("make_universal_regions_live()") ; origin_live_on_entry . reserve (universal_regions . len () * cfg_node . len ()) ; for & origin in universal_regions . iter () { for & point in cfg_node . iter () { origin_live_on_entry . push ((origin , point)) ; } } }
};
}
