// Generated macro for Extension (struct)
macro_rules! Depcrate_fmt_strtimeExtension {
() => {
// Module: crate::fmt::strtime
// Provides: {"Extension"}
// Dependencies: {}
# [doc = " These are \"extensions\" to the standard `strftime` conversion specifiers."] # [doc = ""] # [doc = " This type represents which flags and/or padding were provided with a"] # [doc = " specifier. For example, `%_3d` uses 3 spaces of padding."] # [doc = ""] # [doc = " Currently, this type provides no structured introspection facilities. It"] # [doc = " is exported and available only via implementations of the [`Custom`] trait"] # [doc = " for reasons of semver compatible API evolution. If you have use cases for"] # [doc = " introspecting this type, please open an issue."] # [derive (Clone , Debug)] pub struct Extension { flag : Option < Flag > , width : Option < u8 > , colons : u8 , }
};
}
