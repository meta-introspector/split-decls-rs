macro_rules! deps {
    () => {
        Default!();
        Note!();
        Clone!();
    };
}

macro_rules! State {
    () => {
        deps!();
        # [doc = " A summary of the state of all parts forming a submodule, which allows to answer various questions about it."] # [doc = ""] # [doc = " Note that expensive questions about its presence in the `HEAD` or the `index` are left to the caller."] # [derive (Default , Copy , Clone , Debug , Ord , PartialOrd , Eq , PartialEq , Hash)] pub struct State { # [doc = " if the submodule repository has been cloned."] pub repository_exists : bool , # [doc = " if the submodule repository is located directly in the worktree of the superproject."] pub is_old_form : bool , # [doc = " if the worktree is checked out."] pub worktree_checkout : bool , # [doc = " If submodule configuration was found in the superproject's `.git/config` file."] # [doc = " Note that the presence of a single section is enough, independently of the actual values."] pub superproject_configuration : bool , }
    };
}

State!()