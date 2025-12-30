// Generated macro for octopus (module)
macro_rules! Depcrate_merge_baseoctopus {
() => {
// Module: crate::merge_base
// Provides: {"octopus"}
// Dependencies: {}
mod octopus { use gix_hash :: ObjectId ; use gix_revwalk :: { graph , Graph } ; use crate :: merge_base :: { Error , Flags } ; # [doc = " Given a commit at `first` id, traverse the commit `graph` and return *the best common ancestor* between it and `others`,"] # [doc = " sorted from best to worst. Returns `None` if there is no common merge-base as `first` and `others` don't *all* share history."] # [doc = " If `others` is empty, `Some(first)` is returned."] # [doc = ""] # [doc = " # Performance"] # [doc = ""] # [doc = " For repeated calls, be sure to re-use `graph` as its content will be kept and reused for a great speed-up. The contained flags"] # [doc = " will automatically be cleared."] pub fn octopus (mut first : ObjectId , others : & [ObjectId] , graph : & mut Graph < '_ , '_ , graph :: Commit < Flags > > ,) -> Result < Option < ObjectId > , Error > { for other in others { if let Some (next) = crate :: merge_base (first , std :: slice :: from_ref (other) , graph) ? . and_then (| bases | bases . into_iter () . next ()) { first = next ; } else { return Ok (None) ; } } Ok (Some (first)) } }
};
}
