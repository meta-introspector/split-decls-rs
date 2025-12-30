// Generated macro for impl_172 (impl)
macro_rules! Depcrateimpl_172 {
() => {
// Module: crate
// Provides: {"impl_172"}
// Dependencies: {}
impl TableEntry { pub fn new (string : StringEntry , raw_symbol : String) -> Self { Self { string , raw_symbol } } # [cfg (test)] fn new_without_symbol (tag : Tag , string : String) -> Self { Self { string : StringEntry :: new (tag , string) , raw_symbol : "<unknown>" . to_string () , } } }
};
}
