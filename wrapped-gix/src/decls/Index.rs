macro_rules! Index {
    () => {
        # [doc = " A lazily loaded and auto-updated worktree index."] # [cfg (feature = "index")] pub type Index = gix_fs :: SharedFileSnapshot < gix_index :: File > ;
    };
}

Index!();