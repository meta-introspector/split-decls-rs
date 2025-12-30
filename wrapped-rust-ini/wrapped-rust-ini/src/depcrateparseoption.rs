// Generated macro for ParseOption (struct)
macro_rules! DepcrateParseOption {
() => {
// Module: crate
// Provides: {"ParseOption"}
// Dependencies: {}
# [doc = " Parsing configuration"] pub struct ParseOption { # [doc = " Allow quote (`\"` or `'`) in value"] # [doc = " For example"] # [doc = " ```ini"] # [doc = " [Section]"] # [doc = " Key1=\"Quoted value\""] # [doc = " Key2='Single Quote' with extra value"] # [doc = " ```"] # [doc = ""] # [doc = " In this example, Value of `Key1` is `Quoted value`,"] # [doc = " and value of `Key2` is `Single Quote with extra value`"] # [doc = " if `enabled_quote` is set to `true`."] pub enabled_quote : bool , # [doc = " Interpret `\\` as an escape character"] # [doc = " For example"] # [doc = " ```ini"] # [doc = " [Section]"] # [doc = " Key1=C:\\Windows"] # [doc = " ```"] # [doc = ""] # [doc = " If `enabled_escape` is true, then the value of `Key` will become `C:Windows` (`\\W` equals to `W`)."] pub enabled_escape : bool , # [doc = " Enables values that span lines"] # [doc = " ```ini"] # [doc = " [Section]"] # [doc = " foo="] # [doc = "   b"] # [doc = "   c"] # [doc = " ```"] pub enabled_indented_mutiline_value : bool , # [doc = " Preserve key leading whitespace"] # [doc = ""] # [doc = " ```ini"] # [doc = " [services my-services]"] # [doc = " dynamodb="] # [doc = "   endpoint_url=http://localhost:8000"] # [doc = " ```"] # [doc = ""] # [doc = " The leading whitespace in key `  endpoint_url` will be preserved if `enabled_preserve_key_leading_whitespace` is set to `true`."] pub enabled_preserve_key_leading_whitespace : bool , }
};
}
