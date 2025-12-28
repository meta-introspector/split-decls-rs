macro_rules! deps {
    () => {
        PatternError!();
        MatchOptions!();
        Paths!();
        GlobResult!();
    };
}

macro_rules! glob {
    () => {
        deps!();
        # [doc = " Return an iterator that produces all the `Path`s that match the given"] # [doc = " pattern using default match options, which may be absolute or relative to"] # [doc = " the current working directory."] # [doc = ""] # [doc = " This may return an error if the pattern is invalid."] # [doc = ""] # [doc = " This method uses the default match options and is equivalent to calling"] # [doc = " `glob_with(pattern, MatchOptions::new())`. Use `glob_with` directly if you"] # [doc = " want to use non-default match options."] # [doc = ""] # [doc = " When iterating, each result is a `GlobResult` which expresses the"] # [doc = " possibility that there was an `IoError` when attempting to read the contents"] # [doc = " of the matched path.  In other words, each item returned by the iterator"] # [doc = " will either be an `Ok(Path)` if the path matched, or an `Err(GlobError)` if"] # [doc = " the path (partially) matched _but_ its contents could not be read in order"] # [doc = " to determine if its contents matched."] # [doc = ""] # [doc = " See the `Paths` documentation for more information."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Consider a directory `/media/pictures` containing only the files"] # [doc = " `kittens.jpg`, `puppies.jpg` and `hamsters.gif`:"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use glob::glob;"] # [doc = ""] # [doc = " for entry in glob(\"/media/pictures/*.jpg\").unwrap() {"] # [doc = "     match entry {"] # [doc = "         Ok(path) => println!(\"{:?}\", path.display()),"] # [doc = ""] # [doc = "         // if the path matched but was unreadable,"] # [doc = "         // thereby preventing its contents from matching"] # [doc = "         Err(e) => println!(\"{:?}\", e),"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " The above code will print:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " /media/pictures/kittens.jpg"] # [doc = " /media/pictures/puppies.jpg"] # [doc = " ```"] # [doc = ""] # [doc = " If you want to ignore unreadable paths, you can use something like"] # [doc = " `filter_map`:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use glob::glob;"] # [doc = " use std::result::Result;"] # [doc = ""] # [doc = " for path in glob(\"/media/pictures/*.jpg\").unwrap().filter_map(Result::ok) {"] # [doc = "     println!(\"{}\", path.display());"] # [doc = " }"] # [doc = " ```"] # [doc = " Paths are yielded in alphabetical order."] pub fn glob (pattern : & str) -> Result < Paths , PatternError > { glob_with (pattern , MatchOptions :: new ()) }
    };
}

glob!()