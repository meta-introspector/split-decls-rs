macro_rules! deps {
    () => {
        Pattern!();
        MatchOptions!();
        PathWrapper!();
        GlobError!();
        GlobResult!();
    };
}

macro_rules! Paths {
    () => {
        deps!();
        # [doc = " An iterator that yields `Path`s from the filesystem that match a particular"] # [doc = " pattern."] # [doc = ""] # [doc = " Note that it yields `GlobResult` in order to report any `IoErrors` that may"] # [doc = " arise during iteration. If a directory matches but is unreadable,"] # [doc = " thereby preventing its contents from being checked for matches, a"] # [doc = " `GlobError` is returned to express this."] # [doc = ""] # [doc = " See the `glob` function for more details."] # [derive (Debug)] pub struct Paths { dir_patterns : Vec < Pattern > , require_dir : bool , options : MatchOptions , todo : Vec < Result < (PathWrapper , usize) , GlobError > > , scope : Option < PathWrapper > , }
    };
}

Paths!()