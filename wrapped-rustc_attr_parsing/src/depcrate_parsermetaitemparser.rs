// Generated macro for MetaItemParser (struct)
macro_rules! Depcrate_parserMetaItemParser {
() => {
// Module: crate::parser
// Provides: {"MetaItemParser"}
// Dependencies: {}
# [doc = " Utility that deconstructs a MetaItem into usable parts."] # [doc = ""] # [doc = " MetaItems are syntactically extremely flexible, but specific attributes want to parse"] # [doc = " them in custom, more restricted ways. This can be done using this struct."] # [doc = ""] # [doc = " MetaItems consist of some path, and some args. The args could be empty. In other words:"] # [doc = ""] # [doc = " - `name` -> args are empty"] # [doc = " - `name(...)` -> args are a [`list`](ArgParser::list), which is the bit between the parentheses"] # [doc = " - `name = value`-> arg is [`name_value`](ArgParser::name_value), where the argument is the"] # [doc = "   `= value` part"] # [doc = ""] # [doc = " The syntax of MetaItems can be found at <https://doc.rust-lang.org/reference/attributes.html>"] # [derive (Clone)] pub struct MetaItemParser < 'a > { path : PathParser < 'a > , args : ArgParser < 'a > , }
};
}
