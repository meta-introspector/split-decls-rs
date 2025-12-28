macro_rules! IndexMatchedPath {
    () => {
        # [doc = " A callback function to filter index matches."] # [doc = ""] # [doc = " Used by `Index::{add_all,remove_all,update_all}`.  The first argument is the"] # [doc = " path, and the second is the pathspec that matched it.  Return 0 to confirm"] # [doc = " the operation on the item, > 0 to skip the item, and < 0 to abort the scan."] pub type IndexMatchedPath < 'a > = dyn FnMut (& Path , & [u8]) -> i32 + 'a ;
    };
}

IndexMatchedPath!()