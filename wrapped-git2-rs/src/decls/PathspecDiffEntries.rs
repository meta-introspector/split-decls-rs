macro_rules! deps {
    () => {
        PathspecMatchList!();
    };
}

macro_rules! PathspecDiffEntries {
    () => {
        deps!();
        # [doc = " Iterator over the matching diff deltas."] pub struct PathspecDiffEntries < 'list > { range : Range < usize > , list : & 'list PathspecMatchList < 'list > , }
    };
}

PathspecDiffEntries!();