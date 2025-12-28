macro_rules! deps {
    () => {
        Tree!();
        Editor!();
        Kind!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl < 'a > Editor < 'a > { # [doc = " Create a new editor that uses `root` as base for all edits. Use `find` to lookup existing"] # [doc = " trees when edits are made. Each tree will only be looked-up once and then edited in place from"] # [doc = " that point on."] # [doc = " `object_hash` denotes the kind of hash to create."] pub fn new (root : Tree , find : & 'a dyn crate :: FindExt , object_hash : gix_hash :: Kind) -> Self { Editor { find , object_hash , trees : HashMap :: from_iter (Some ((empty_path () , root))) , path_buf : BString :: from (Vec :: with_capacity (256)) . into () , tree_buf : Vec :: with_capacity (512) , } } }
    };
}

impl_105!()