macro_rules! Matches {
    () => {
        # [doc = " Parallel iterator over substrings that match a pattern"] # [derive (Debug , Clone)] pub struct Matches < 'ch , P : Pattern > { chars : & 'ch str , pattern : P , }
    };
}

Matches!()