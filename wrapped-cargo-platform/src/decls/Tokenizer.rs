macro_rules! Tokenizer {
    () => {
        # [derive (Clone)] struct Tokenizer < 'a > { s : iter :: Peekable < str :: CharIndices < 'a > > , orig : & 'a str , }
    };
}

Tokenizer!()