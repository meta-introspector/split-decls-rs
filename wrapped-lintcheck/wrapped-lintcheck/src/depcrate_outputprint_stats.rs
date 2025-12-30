// Generated macro for print_stats (function)
macro_rules! Depcrate_outputprint_stats {
() => {
// Module: crate::output
// Provides: {"print_stats"}
// Dependencies: {}
# [doc = " print how lint counts changed between runs"] fn print_stats (old_stats : HashMap < String , usize > , new_stats : HashMap < & String , usize > , lint_filter : & [String]) { let same_in_both_hashmaps = old_stats . iter () . filter (| (old_key , old_val) | new_stats . get :: < & String > (old_key) == Some (old_val)) . map (| (k , v) | (k . clone () , * v)) . collect :: < Vec < (String , usize) > > () ; let mut old_stats_deduped = old_stats ; let mut new_stats_deduped = new_stats ; for (k , v) in & same_in_both_hashmaps { assert ! (old_stats_deduped . remove (k) == Some (* v)) ; assert ! (new_stats_deduped . remove (k) == Some (* v)) ; } println ! ("\nStats:") ; new_stats_deduped . iter () . filter (| (new_key , _) | ! old_stats_deduped . contains_key :: < str > (new_key)) . for_each (| (new_key , new_value) | { println ! ("{new_key} 0 => {new_value}") ; }) ; new_stats_deduped . iter () . filter (| (new_key , _new_val) | old_stats_deduped . contains_key :: < str > (new_key)) . for_each (| (new_key , new_val) | { let old_val = old_stats_deduped . get :: < str > (new_key) . unwrap () ; println ! ("{new_key} {old_val} => {new_val}") ; }) ; old_stats_deduped . iter () . filter (| (old_key , _) | ! new_stats_deduped . contains_key :: < & String > (old_key)) . filter (| (old_key , _) | lint_filter . is_empty () || lint_filter . contains (old_key)) . for_each (| (old_key , old_value) | { println ! ("{old_key} {old_value} => 0") ; }) ; }
};
}
