// Generated macro for Default (enum)
macro_rules! Depcrate_pushDefault {
() => {
// Module: crate::push
// Provides: {"Default"}
// Dependencies: {}
# [doc = " All possible values of `push.default`."] # [derive (Default , Copy , Clone , PartialOrd , PartialEq , Ord , Eq , Hash , Debug)] pub enum Default { # [doc = " Do not push anything unless a refspec is provided explicitly."] # [doc = ""] # [doc = " This is for safety."] Nothing , # [doc = " Push the current branch to update a remote branch with the same name."] Current , # [doc = " Push the current branch to the branch it would fetch from and merge with,"] # [doc = " i.e. what is configured in `branch.<name>.merge`, retrievable with"] # [doc = " the `@{upstream}` refspec."] Upstream , # [doc = " Push the current branch with the same name to the remote."] # [doc = " This is the same as [`Current`](Default::Current), but fails if"] # [doc = " `branch.<name>.merge` is set to a branch that is named differently."] # [default] Simple , # [doc = " Push *all* branches to their similarly named counterpart on the remote."] Matching , }
};
}
