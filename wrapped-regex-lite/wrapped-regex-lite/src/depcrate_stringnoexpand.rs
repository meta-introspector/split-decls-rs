// Generated macro for NoExpand (struct)
macro_rules! Depcrate_stringNoExpand {
() => {
// Module: crate::string
// Provides: {"NoExpand"}
// Dependencies: {}
# [doc = " A helper type for forcing literal string replacement."] # [doc = ""] # [doc = " It can be used with routines like [`Regex::replace`] and"] # [doc = " [`Regex::replace_all`] to do a literal string replacement without expanding"] # [doc = " `$name` to their corresponding capture groups. This can be both convenient"] # [doc = " (to avoid escaping `$`, for example) and faster (since capture groups"] # [doc = " don't need to be found)."] # [doc = ""] # [doc = " `'s` is the lifetime of the literal string to use."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use regex_lite::{NoExpand, Regex};"] # [doc = ""] # [doc = " let re = Regex::new(r\"(?<last>[^,\\s]+),\\s+(\\S+)\").unwrap();"] # [doc = " let result = re.replace(\"Springsteen, Bruce\", NoExpand(\"$2 $last\"));"] # [doc = " assert_eq!(result, \"$2 $last\");"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct NoExpand < 't > (pub & 't str) ;
};
}
