// Generated macro for all_except_most_recent (function)
macro_rules! Depcrate_persist_fsall_except_most_recent {
() => {
// Module: crate::persist::fs
// Provides: {"all_except_most_recent"}
// Dependencies: {}
fn all_except_most_recent (deletion_candidates : UnordMap < (SystemTime , PathBuf) , Option < flock :: Lock > > ,) -> UnordMap < PathBuf , Option < flock :: Lock > > { let most_recent = deletion_candidates . items () . map (| (& (timestamp , _) , _) | timestamp) . max () ; if let Some (most_recent) = most_recent { deletion_candidates . into_items () . filter (| & ((timestamp , _) , _) | timestamp != most_recent) . map (| ((_ , path) , lock) | (path , lock)) . collect () } else { UnordMap :: default () } }
};
}
