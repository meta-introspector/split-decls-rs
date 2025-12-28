macro_rules! Prefixes {
    () => {
        # [doc = " Iterator of all non-empty prefixes"] # [derive (Clone)] pub struct Prefixes < 'a > { s : & 'a str , iter : str :: CharIndices < 'a > , }
    };
}

Prefixes!();