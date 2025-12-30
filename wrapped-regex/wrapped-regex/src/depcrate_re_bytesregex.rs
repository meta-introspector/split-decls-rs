// Generated macro for Regex (struct)
macro_rules! Depcrate_re_bytesRegex {
() => {
// Module: crate::re_bytes
// Provides: {"Regex"}
// Dependencies: {}
# [doc = " A compiled regular expression for matching arbitrary bytes."] # [doc = ""] # [doc = " It can be used to search, split or replace text. All searching is done with"] # [doc = " an implicit `.*?` at the beginning and end of an expression. To force an"] # [doc = " expression to match the whole string (or a prefix or a suffix), you must"] # [doc = " use an anchor like `^` or `$` (or `\\A` and `\\z`)."] # [doc = ""] # [doc = " Like the `Regex` type in the parent module, matches with this regex return"] # [doc = " byte offsets into the search text. **Unlike** the parent `Regex` type,"] # [doc = " these byte offsets may not correspond to UTF-8 sequence boundaries since"] # [doc = " the regexes in this module can match arbitrary bytes."] # [derive (Clone)] pub struct Regex (Exec) ;
};
}
