macro_rules! GlobWalker {
    () => {
        # [doc = " An iterator which emits glob-matched patterns."] # [doc = ""] # [doc = " An instance of this type must be constructed through `GlobWalker`,"] # [doc = " which uses a builder-style pattern."] # [doc = ""] # [doc = " The order of the yielded paths is undefined, unless specified by the user"] # [doc = " using `GlobWalker::sort_by`."] pub struct GlobWalker { ignore : Override , walker : walkdir :: IntoIter , file_type_filter : Option < FileType > , }
    };
}

GlobWalker!();