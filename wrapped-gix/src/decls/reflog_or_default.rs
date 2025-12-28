macro_rules! reflog_or_default {
    () => {
        pub (crate) fn reflog_or_default (config_reflog : Option < gix_ref :: store :: WriteReflog > , has_worktree : bool ,) -> gix_ref :: store :: WriteReflog { config_reflog . unwrap_or (if has_worktree { gix_ref :: store :: WriteReflog :: Normal } else { gix_ref :: store :: WriteReflog :: Disable }) }
    };
}

reflog_or_default!()