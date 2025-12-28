macro_rules! MatchIndices {
    () => {
        # [doc = " Parallel iterator over substrings that match a pattern, with their positions"] # [derive (Debug , Clone)] pub struct MatchIndices < 'ch , P : Pattern > { chars : & 'ch str , pattern : P , }
    };
}

MatchIndices!()