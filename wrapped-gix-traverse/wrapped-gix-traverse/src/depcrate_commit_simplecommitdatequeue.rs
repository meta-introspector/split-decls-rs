// Generated macro for CommitDateQueue (type)
macro_rules! Depcrate_commit_simpleCommitDateQueue {
() => {
// Module: crate::commit::simple
// Provides: {"CommitDateQueue"}
// Dependencies: {}
type CommitDateQueue = gix_revwalk :: PriorityQueue < QueueKey < SecondsSinceUnixEpoch > , (ObjectId , CommitState) > ;
};
}
