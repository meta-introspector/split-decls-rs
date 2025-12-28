macro_rules! deps {
    () => {
        Kind!();
        Tree!();
    };
}

macro_rules! Editor {
    () => {
        deps!();
        # [doc = " The state needed to apply edits instantly to in-memory trees."] # [doc = ""] # [doc = " It's made so that each tree is looked at in the object database at most once, and held in memory for"] # [doc = " all edits until everything is flushed to write all changed trees."] # [doc = ""] # [doc = " The editor is optimized to edit existing trees, but can deal with building entirely new trees as well"] # [doc = " with some penalties."] # [doc (alias = "TreeUpdateBuilder" , alias = "git2")] # [derive (Clone)] pub struct Editor < 'a > { # [doc = " A way to lookup trees."] find : & 'a dyn crate :: FindExt , # [doc = " The kind of hashes to produce>"] object_hash : gix_hash :: Kind , # [doc = " All trees we currently hold in memory. Each of these may change while adding and removing entries."] # [doc = " null-object-ids mark tree-entries whose value we don't know yet, they are placeholders that will be"] # [doc = " dropped when writing at the latest."] trees : std :: collections :: HashMap < BString , Tree > , # [doc = " A buffer to build up paths when finding the tree to edit."] path_buf : RefCell < BString > , # [doc = " Our buffer for storing tree-data in, right before decoding it."] tree_buf : Vec < u8 > , }
    };
}

Editor!()