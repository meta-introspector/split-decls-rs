macro_rules! deps {
    () => {
        Options!();
        Path!();
        Note!();
    };
}

macro_rules! impl_446 {
    () => {
        deps!();
        # [doc = " Setters"] impl Options { # [doc = " Do not keep track of filepaths at all, which will leave all `location` fields empty."] pub fn no_locations (& mut self) -> & mut Self { self . location = None ; self } # [doc = " Keep track of file-names, which makes `location` fields usable with the filename of the changed item."] pub fn track_filename (& mut self) -> & mut Self { self . location = Some (Location :: FileName) ; self } # [doc = " Keep track of the entire path of a change, relative to the repository. (default)."] # [doc = ""] # [doc = " This makes the `location` field fully usable."] pub fn track_path (& mut self) -> & mut Self { self . location = Some (Location :: Path) ; self } # [doc = " Provide `None` to disable rewrite tracking entirely, or pass `Some(<configuration>)` to control to"] # [doc = " what extent rename and copy tracking is performed."] # [doc = ""] # [doc = " Note that by default, the git configuration determines rewrite tracking and git defaults are used"] # [doc = " if nothing is configured, which turns rename tracking with 50% similarity on, while not tracking copies at all."] # [cfg (feature = "blob-diff")] pub fn track_rewrites (& mut self , renames : Option < gix_diff :: Rewrites >) -> & mut Self { self . rewrites = renames ; self } }
    };
}

impl_446!()