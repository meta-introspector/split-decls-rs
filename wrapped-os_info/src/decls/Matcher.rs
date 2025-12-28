macro_rules! Matcher {
    () => {
        # [doc = " An implementation to match on simple strings."] # [derive (Debug , Clone)] # [allow (dead_code)] pub enum Matcher { # [doc = " Considers the entire string (trimmed) to be the match."] AllTrimmed , # [doc = " After finding the `prefix` followed by one or more spaces, returns the following word."] PrefixedWord { prefix : & 'static str } , # [doc = " Similar to `PrefixedWord`, but only if the word is a valid version."] PrefixedVersion { prefix : & 'static str } , # [doc = " Takes a set of lines (separated by `\\n`) and searches for the value in a key/value pair"] # [doc = " separated by the `=` character. For example `VERSION_ID=\"8.1\"`."] KeyValue { key : & 'static str } , }
    };
}

Matcher!()