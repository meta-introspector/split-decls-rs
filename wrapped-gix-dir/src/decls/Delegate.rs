macro_rules! deps {
    () => {
        Action!();
        Status!();
        EntryRef!();
        ForDeletionMode!();
    };
}

macro_rules! Delegate {
    () => {
        deps!();
        # [doc = " A way for the caller to control the traversal based on provided data."] pub trait Delegate { # [doc = " Called for each observed `entry` *inside* a directory, or the directory itself if the traversal is configured"] # [doc = " to simplify the result (i.e. if every file in a directory is ignored, emit the containing directory instead"] # [doc = " of each file), or if the root of the traversal passes through a directory that can't be traversed."] # [doc = ""] # [doc = " It will also be called if the `root` in [`walk()`](crate::walk()) itself is matching a particular status,"] # [doc = " even if it is a file."] # [doc = ""] # [doc = " Note that tracked entries will only be emitted if [`Options::emit_tracked`] is `true`."] # [doc = " Further, not all pruned entries will be observable as they might be pruned so early that the kind of"] # [doc = " item isn't yet known. Pruned entries are also only emitted if [`Options::emit_pruned`] is `true`."] # [doc = ""] # [doc = " `collapsed_directory_status` is `Some(dir_status)` if this entry was part of a directory with the given"] # [doc = " `dir_status` that wasn't the same as the one of `entry` and if [Options::emit_collapsed] was"] # [doc = " [CollapsedEntriesEmissionMode::OnStatusMismatch]. It will also be `Some(dir_status)` if that option"] # [doc = " was [CollapsedEntriesEmissionMode::All]."] fn emit (& mut self , entry : EntryRef < '_ > , collapsed_directory_status : Option < entry :: Status >) -> Action ; # [doc = " Return `true` if the given entry can be recursed into. Will only be called if the entry is a physical directory."] # [doc = " The base implementation will act like Git does by default in `git status` or `git clean`."] # [doc = ""] # [doc = " Use `for_deletion` to specify if the seen entries should ultimately be deleted, which may affect the decision"] # [doc = " of whether to resource or not."] # [doc = ""] # [doc = " If `worktree_root_is_repository` is `true`, then this status is part of the root of an iteration, and the corresponding"] # [doc = " worktree root is a repository itself. This typically happens for submodules. In this case, recursion rules are relaxed"] # [doc = " to allow traversing submodule worktrees."] # [doc = ""] # [doc = " Note that this method will see all directories, even though not all of them may end up being [emitted](Self::emit())."] # [doc = " If this method returns `false`, the `entry` will always be emitted."] fn can_recurse (& mut self , entry : EntryRef < '_ > , for_deletion : Option < ForDeletionMode > , worktree_root_is_repository : bool ,) -> bool { entry . status . can_recurse (entry . disk_kind , entry . pathspec_match , for_deletion , worktree_root_is_repository ,) } }
    };
}

Delegate!();