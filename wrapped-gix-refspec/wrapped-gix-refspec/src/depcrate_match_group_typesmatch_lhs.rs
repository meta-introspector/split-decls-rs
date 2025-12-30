// Generated macro for match_lhs (module)
macro_rules! Depcrate_match_group_typesmatch_lhs {
() => {
// Module: crate::match_group::types
// Provides: {"match_lhs"}
// Dependencies: {}
# [doc = ""] pub mod match_lhs { use crate :: { match_group :: Mapping , MatchGroup } ; # [doc = " The outcome of any matching operation of a [`MatchGroup`]."] # [doc = ""] # [doc = " It's used to validate and process the contained [mappings](Mapping)."] # [derive (Debug , Clone)] pub struct Outcome < 'spec , 'item > { # [doc = " The match group that produced this outcome."] pub group : MatchGroup < 'spec > , # [doc = " The mappings derived from matching [items](crate::match_group::Item)."] pub mappings : Vec < Mapping < 'item , 'spec > > , } }
};
}
