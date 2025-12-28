macro_rules! deps {
    () => {
        TreeBuilder!();
    };
}

macro_rules! TreeUpdateBuilder {
    () => {
        deps!();
        # [doc = " A builder struct for git tree updates."] # [doc = ""] # [doc = " Paths passed to `remove` and `upsert` can be multi-component paths, i.e. they"] # [doc = " may contain slashes."] # [doc = ""] # [doc = " This is a higher-level tree update facility.  There is also [`TreeBuilder`]"] # [doc = " which is lower-level (and operates only on one level of the tree at a time)."] # [doc = ""] # [doc = " [`TreeBuilder`]: crate::TreeBuilder"] pub struct TreeUpdateBuilder { updates : Vec < raw :: git_tree_update > , paths : Vec < CString > , }
    };
}

TreeUpdateBuilder!()