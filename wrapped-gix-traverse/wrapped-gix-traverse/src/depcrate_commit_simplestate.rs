// Generated macro for State (struct)
macro_rules! Depcrate_commit_simpleState {
() => {
// Module: crate::commit::simple
// Provides: {"State"}
// Dependencies: {}
# [doc = " The state used and potentially shared by multiple graph traversals."] # [derive (Clone)] pub (super) struct State { next : VecDeque < (ObjectId , CommitState) > , queue : CommitDateQueue , buf : Vec < u8 > , seen : gix_revwalk :: graph :: IdMap < CommitState > , parents_buf : Vec < u8 > , parent_ids : SmallVec < (ObjectId , SecondsSinceUnixEpoch) , 2 > , # [doc = " The list (FIFO) of thus far interesting commits."] # [doc = ""] # [doc = " As they may turn hidden later, we have to keep them until the conditions are met to return them."] # [doc = " If `None`, there is nothing to do with hidden commits."] candidates : Option < Candidates > , }
};
}
