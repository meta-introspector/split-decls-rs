macro_rules! Extensions {
    () => {
        # [doc = " A way to specify which of the optional extensions to write."] # [derive (Default , Debug , Copy , Clone)] pub enum Extensions { # [doc = " Writes all available optional extensions to avoid losing any information."] # [default] All , # [doc = " Only write the given optional extensions, with each extension being marked by a boolean flag."] # [doc = ""] # [doc = " # Note: mandatory extensions"] # [doc = ""] # [doc = " Mandatory extensions, like `sdir` or other lower-case ones, may not be configured here as they need to be present"] # [doc = " or absent depending on the state of the index itself and for it to be valid."] Given { # [doc = " Write the tree-cache extension, if present."] tree_cache : bool , # [doc = " Write the end-of-index-entry extension."] end_of_index_entry : bool , } , # [doc = " Write no optional extension at all for what should be the smallest possible index"] None , }
    };
}

Extensions!();