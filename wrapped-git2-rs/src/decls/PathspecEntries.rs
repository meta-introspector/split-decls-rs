macro_rules! deps {
    () => {
        PathspecMatchList!();
    };
}

macro_rules! PathspecEntries {
    () => {
        deps!();
        # [doc = " Iterator over the matched paths in a pathspec."] pub struct PathspecEntries < 'list > { range : Range < usize > , list : & 'list PathspecMatchList < 'list > , }
    };
}

PathspecEntries!();