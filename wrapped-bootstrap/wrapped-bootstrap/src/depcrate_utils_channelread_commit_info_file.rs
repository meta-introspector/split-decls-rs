// Generated macro for read_commit_info_file (function)
macro_rules! Depcrate_utils_channelread_commit_info_file {
() => {
// Module: crate::utils::channel
// Provides: {"read_commit_info_file"}
// Dependencies: {}
# [doc = " Read the commit information from the `git-commit-info` file given the"] # [doc = " project root."] pub fn read_commit_info_file (root : & Path) -> Option < Info > { if let Ok (contents) = fs :: read_to_string (root . join ("git-commit-info")) { let mut lines = contents . lines () ; let sha = lines . next () ; let short_sha = lines . next () ; let commit_date = lines . next () ; let info = match (commit_date , sha , short_sha) { (Some (commit_date) , Some (sha) , Some (short_sha)) => Info { commit_date : commit_date . to_owned () , sha : sha . to_owned () , short_sha : short_sha . to_owned () , } , _ => panic ! ("the `git-commit-info` file is malformed") , } ; Some (info) } else { None } }
};
}
