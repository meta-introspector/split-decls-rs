macro_rules! deps {
    () => {
        Pathspec!();
    };
}

macro_rules! PathspecMatchList {
    () => {
        deps!();
        # [doc = " List of filenames matching a pathspec."] pub struct PathspecMatchList < 'ps > { raw : * mut raw :: git_pathspec_match_list , _marker : marker :: PhantomData < & 'ps Pathspec > , }
    };
}

PathspecMatchList!()