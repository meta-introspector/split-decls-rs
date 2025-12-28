macro_rules! CharIndices {
    () => {
        # [doc = " Parallel iterator over the characters of a string, with their positions"] # [derive (Debug , Clone)] pub struct CharIndices < 'ch > { chars : & 'ch str , }
    };
}

CharIndices!();