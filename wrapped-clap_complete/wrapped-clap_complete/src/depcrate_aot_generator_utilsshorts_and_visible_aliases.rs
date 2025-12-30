// Generated macro for shorts_and_visible_aliases (function)
macro_rules! Depcrate_aot_generator_utilsshorts_and_visible_aliases {
() => {
// Module: crate::aot::generator::utils
// Provides: {"shorts_and_visible_aliases"}
// Dependencies: {}
# [doc = " Gets all the short options, their visible aliases and flags of a [`clap::Command`]."] # [doc = " Includes `h` and `V` depending on the [`clap::Command`] settings."] pub fn shorts_and_visible_aliases (p : & Command) -> Vec < char > { debug ! ("shorts: name={}" , p . get_name ()) ; p . get_arguments () . filter_map (| a | { if ! a . is_positional () { if a . get_visible_short_aliases () . is_some () && a . get_short () . is_some () { let mut shorts_and_visible_aliases = a . get_visible_short_aliases () . unwrap () ; shorts_and_visible_aliases . push (a . get_short () . unwrap ()) ; Some (shorts_and_visible_aliases) } else if a . get_visible_short_aliases () . is_none () && a . get_short () . is_some () { Some (vec ! [a . get_short () . unwrap ()]) } else { None } } else { None } }) . flatten () . collect () }
};
}
