macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! Resource {
    () => {
        deps!();
        # [doc = " A stored value representing a resource that participates in a merge."] # [derive (Clone , Eq , PartialEq , Ord , PartialOrd , Debug)] pub (super) struct Resource { # [doc = " The `id` of the value, or `null` if it's only living in a worktree."] id : gix_hash :: ObjectId , # [doc = " The repository-relative path where the resource lives in the tree."] rela_path : BString , # [doc = " The outcome of converting a resource into a mergable format using [Pipeline::convert_to_mergeable()]."] data : Option < pipeline :: Data > , # [doc = " The kind of the resource we are looking at. Only possible values are `Blob` and `BlobExecutable`."] mode : gix_object :: tree :: EntryKind , # [doc = " A possibly empty buffer, depending on `conversion.data` which may indicate the data is considered binary"] # [doc = " or the resource doesn't exist."] buffer : Vec < u8 > , }
    };
}

Resource!()