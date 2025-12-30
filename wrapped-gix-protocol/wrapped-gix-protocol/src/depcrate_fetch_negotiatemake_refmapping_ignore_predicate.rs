// Generated macro for make_refmapping_ignore_predicate (function)
macro_rules! Depcrate_fetch_negotiatemake_refmapping_ignore_predicate {
() => {
// Module: crate::fetch::negotiate
// Provides: {"make_refmapping_ignore_predicate"}
// Dependencies: {}
# [doc = " Create a predicate that checks if a refspec mapping should be ignored."] # [doc = ""] # [doc = " We want to ignore mappings during negotiation if they would be handled implicitly by the server, which is the case"] # [doc = " when tags would be sent implicitly due to `Tags::Included`."] pub fn make_refmapping_ignore_predicate (fetch_tags : Tags , ref_map : & RefMap) -> impl Fn (& refmap :: Mapping) -> bool + '_ { let tag_refspec_to_ignore = matches ! (fetch_tags , Tags :: Included) . then (| | fetch_tags . to_refspec ()) . flatten () ; move | mapping | { tag_refspec_to_ignore . is_some_and (| tag_spec | { mapping . spec_index . implicit_index () . and_then (| idx | ref_map . extra_refspecs . get (idx)) . is_some_and (| spec | spec . to_ref () == tag_spec) }) } }
};
}
