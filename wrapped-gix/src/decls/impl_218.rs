macro_rules! deps {
    () => {
        Stats!();
        Error!();
        Action!();
        Platform!();
        Tree!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        # [doc = " Convenience"] impl Platform < '_ , '_ > { # [doc = " Calculate statistics about the lines of the diff between our current and the `other` tree."] # [doc = ""] # [doc = " ### Performance Notes"] # [doc = ""] # [doc = " Be sure to forcefully disable [`track_rewrites(None)`](crate::diff::Options::track_rewrites) to avoid"] # [doc = " rename tracking, an operation that doesn't affect the statistics currently."] # [doc = " As diffed resources aren't cached, if highly repetitive blobs are expected, performance"] # [doc = " may be diminished. In real-world scenarios where blobs are mostly unique, that's not an issue though."] pub fn stats (& mut self , other : & Tree < '_ >) -> Result < Stats , stats :: Error > { let mut resource_cache = self . lhs . repo . diff_resource_cache_for_tree_diff () ? ; let (mut files_changed , mut lines_added , mut lines_removed) = (0 , 0 , 0) ; self . for_each_to_obtain_tree (other , | change | { if let Some (counts) = change . diff (& mut resource_cache) . ok () . and_then (| mut platform | platform . line_counts () . ok ()) . flatten () { files_changed += 1 ; lines_added += u64 :: from (counts . 1) ; lines_removed += u64 :: from (counts . 0) ; } resource_cache . clear_resource_cache_keep_allocation () ; Ok :: < _ , std :: convert :: Infallible > (Action :: Continue) }) ? ; Ok (Stats { files_changed , lines_added , lines_removed , }) } }
    };
}

impl_218!();