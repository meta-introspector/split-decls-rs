macro_rules! MAX_COMMITS {
    () => {
        # [doc = " The maximum number of commits that can be stored in a commit graph."] pub const MAX_COMMITS : u32 = (1 << 30) + (1 << 29) + (1 << 28) - 1 ;
    };
}

MAX_COMMITS!();