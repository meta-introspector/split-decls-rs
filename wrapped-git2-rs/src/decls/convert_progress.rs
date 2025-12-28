macro_rules! deps {
    () => {
        StashApplyProgress!();
    };
}

macro_rules! convert_progress {
    () => {
        deps!();
        fn convert_progress (progress : raw :: git_stash_apply_progress_t) -> StashApplyProgress { match progress { raw :: GIT_STASH_APPLY_PROGRESS_NONE => StashApplyProgress :: None , raw :: GIT_STASH_APPLY_PROGRESS_LOADING_STASH => StashApplyProgress :: LoadingStash , raw :: GIT_STASH_APPLY_PROGRESS_ANALYZE_INDEX => StashApplyProgress :: AnalyzeIndex , raw :: GIT_STASH_APPLY_PROGRESS_ANALYZE_MODIFIED => StashApplyProgress :: AnalyzeModified , raw :: GIT_STASH_APPLY_PROGRESS_ANALYZE_UNTRACKED => StashApplyProgress :: AnalyzeUntracked , raw :: GIT_STASH_APPLY_PROGRESS_CHECKOUT_UNTRACKED => StashApplyProgress :: CheckoutUntracked , raw :: GIT_STASH_APPLY_PROGRESS_CHECKOUT_MODIFIED => StashApplyProgress :: CheckoutModified , raw :: GIT_STASH_APPLY_PROGRESS_DONE => StashApplyProgress :: Done , _ => StashApplyProgress :: None , } }
    };
}

convert_progress!();