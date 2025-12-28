macro_rules! deps {
    () => {
        Matcher!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Matcher { # [doc = " Find the match on the input `string`."] pub fn find (& self , string : & str) -> Option < String > { match * self { Self :: AllTrimmed => Some (string . trim () . to_string ()) , Self :: PrefixedWord { prefix } => find_prefixed_word (string , prefix) . map (str :: to_owned) , Self :: PrefixedVersion { prefix } => find_prefixed_word (string , prefix) . filter (| & v | is_valid_version (v)) . map (str :: to_owned) , Self :: KeyValue { key } => find_by_key (string , key) . map (str :: to_owned) , } } }
    };
}

impl_34!()