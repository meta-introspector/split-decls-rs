macro_rules! SplitTerminator {
    () => {
        # [doc = " Parallel iterator over substrings separated by a terminator pattern"] # [derive (Debug , Clone)] pub struct SplitTerminator < 'ch , P : Pattern > { chars : & 'ch str , terminator : P , }
    };
}

SplitTerminator!();