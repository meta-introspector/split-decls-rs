macro_rules! deps {
    () => {
        PathspecMatchList!();
    };
}

macro_rules! PathspecFailedEntries {
    () => {
        deps!();
        # [doc = " Iterator over the failed list of pathspec items that did not match."] pub struct PathspecFailedEntries < 'list > { range : Range < usize > , list : & 'list PathspecMatchList < 'list > , }
    };
}

PathspecFailedEntries!()