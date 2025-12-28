macro_rules! Kind {
    () => {
        # [doc = " The kind of *ignored* item."] # [doc = ""] # [doc = " This classification is obtained when checking if a path matches an ignore pattern."] # [derive (Default , Copy , Clone , Ord , PartialOrd , Eq , PartialEq , Hash , Debug)] pub enum Kind { # [doc = " The item is ignored and will be removed to make place for tracked items that are to be checked out."] # [doc = ""] # [doc = " This is the default for ignored items."] # [doc = " Another way of thinking about this class is to consider these files *trashable*, or talk about them as `ignored-and-expendable`."] # [default] Expendable , # [doc = " An ignored file was additionally marked as *precious* using the `$` prefix to indicate the file shall be kept."] # [doc = ""] # [doc = " This means that precious files are treated like untracked files, which also must not be removed, but won't show up by default"] # [doc = " as they are also ignored."] # [doc = " One can also talk about them as `ignored-and-precious`."] Precious , }
    };
}

Kind!();