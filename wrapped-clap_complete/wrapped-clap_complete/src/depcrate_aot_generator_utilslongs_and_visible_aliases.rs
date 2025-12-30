// Generated macro for longs_and_visible_aliases (function)
macro_rules! Depcrate_aot_generator_utilslongs_and_visible_aliases {
() => {
// Module: crate::aot::generator::utils
// Provides: {"longs_and_visible_aliases"}
// Dependencies: {}
# [doc = " Gets all the long options, their visible aliases and flags of a [`clap::Command`]."] # [doc = " Includes `help` and `version` depending on the [`clap::Command`] settings."] pub fn longs_and_visible_aliases (p : & Command) -> Vec < String > { debug ! ("longs: name={}" , p . get_name ()) ; p . get_arguments () . filter_map (| a | { if ! a . is_positional () { if a . get_visible_aliases () . is_some () && a . get_long () . is_some () { let mut visible_aliases : Vec < _ > = a . get_visible_aliases () . unwrap () . into_iter () . map (| s | s . to_string ()) . collect () ; visible_aliases . push (a . get_long () . unwrap () . to_string ()) ; Some (visible_aliases) } else if a . get_visible_aliases () . is_none () && a . get_long () . is_some () { Some (vec ! [a . get_long () . unwrap () . to_string ()]) } else { None } } else { None } }) . flatten () . collect () }
};
}
