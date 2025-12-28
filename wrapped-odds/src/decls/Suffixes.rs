macro_rules! Suffixes {
    () => {
        # [doc = " Iterator of all non-empty suffixes"] # [derive (Clone)] pub struct Suffixes < 'a > { s : & 'a str , iter : str :: CharIndices < 'a > , }
    };
}

Suffixes!()