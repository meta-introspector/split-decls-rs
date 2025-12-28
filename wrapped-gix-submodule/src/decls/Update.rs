macro_rules! Update {
    () => {
        # [doc = " Determine how `git submodule update` should deal with this submodule to bring it up-to-date with the"] # [doc = " super-project's expectations."] # [derive (Default , Debug , Clone , Hash , PartialOrd , PartialEq , Ord , Eq)] pub enum Update { # [doc = " The commit recorded in the superproject should be checked out on a detached `HEAD`."] # [default] Checkout , # [doc = " The current branch in the submodule will be rebased onto the commit recorded in the superproject."] Rebase , # [doc = " The commit recorded in the superproject will merged into the current branch of the submodule."] Merge , # [doc = " A custom command to be called like `<command> hash-of-submodule-commit` that is to be executed to"] # [doc = " perform the submodule update."] # [doc = ""] # [doc = " Note that this variant is only allowed if the value is coming from an override. Thus it's not allowed to distribute"] # [doc = " arbitrary commands via `.gitmodules` for security reasons."] Command (BString) , # [doc = " The submodule update is not performed at all."] None , }
    };
}

Update!();