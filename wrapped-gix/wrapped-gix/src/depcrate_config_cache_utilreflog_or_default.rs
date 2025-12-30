// Generated macro for reflog_or_default (function)
macro_rules! Depcrate_config_cache_utilreflog_or_default {
() => {
// Module: crate::config::cache::util
// Provides: {"reflog_or_default"}
// Dependencies: {}
pub (crate) fn reflog_or_default (config_reflog : Option < gix_ref :: store :: WriteReflog > , has_worktree : bool ,) -> gix_ref :: store :: WriteReflog { config_reflog . unwrap_or (if has_worktree { gix_ref :: store :: WriteReflog :: Normal } else { gix_ref :: store :: WriteReflog :: Disable }) }
};
}
