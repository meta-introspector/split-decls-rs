macro_rules! SplitInclusive {
    () => {
        # [doc = " Parallel iterator over substrings separated by a pattern"] # [derive (Debug , Clone)] pub struct SplitInclusive < 'ch , P : Pattern > { chars : & 'ch str , separator : P , }
    };
}

SplitInclusive!()