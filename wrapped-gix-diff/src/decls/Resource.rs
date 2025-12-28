macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! Resource {
    () => {
        deps!();
        # [doc = " A resource ready to be diffed in one way or another."] # [derive (Debug , Copy , Clone , Ord , PartialOrd , Eq , PartialEq , Hash)] pub struct Resource < 'a > { # [doc = " If available, an index into the `drivers` field to access more diff-related information of the driver for items"] # [doc = " at the given path, as previously determined by git-attributes."] # [doc = ""] # [doc = " Note that drivers are queried even if there is no object available."] pub driver_index : Option < usize > , # [doc = " The data itself, suitable for diffing, and if the object or worktree item is present at all."] pub data : resource :: Data < 'a > , # [doc = " The kind of the resource we are looking at. Only possible values are `Blob`, `BlobExecutable` and `Link`."] pub mode : gix_object :: tree :: EntryKind , # [doc = " The location of the resource, relative to the working tree."] pub rela_path : & 'a BStr , # [doc = " The id of the content as it would be stored in `git`, or `null` if the content doesn't exist anymore at"] # [doc = " `rela_path` or if it was never computed. This can happen with content read from the worktree, which has to"] # [doc = " go through a filter to be converted back to what `git` would store."] pub id : & 'a gix_hash :: oid , }
    };
}

Resource!();