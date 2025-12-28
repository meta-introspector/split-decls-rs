macro_rules! FetchRecurse {
    () => {
        # [doc = " Determine how to recurse into this module from the superproject when fetching."] # [doc = ""] # [doc = " Generally, a fetch is only performed if the submodule commit referenced by the superproject isn't already"] # [doc = " present in the submodule repository."] # [doc = ""] # [doc = " Note that when unspecified, the `fetch.recurseSubmodules` configuration variable should be used instead."] # [derive (Default , Debug , Clone , Copy , Ord , PartialOrd , Eq , PartialEq , Hash)] pub enum FetchRecurse { # [doc = " Fetch only changed submodules."] # [default] OnDemand , # [doc = " Fetch all populated submodules, changed or not."] # [doc = ""] # [doc = " This skips the work needed to determine whether a submodule has changed in the first place, but may work"] # [doc = " more as some fetches might not be necessary."] Always , # [doc = " Submodules are never fetched."] Never , }
    };
}

FetchRecurse!();