// Generated macro for insert (function)
macro_rules! Depcrate_reader_item_indexinsert {
() => {
// Module: crate::reader::item_index
// Provides: {"insert"}
// Dependencies: {}
fn insert < 'a > (members : & mut HashType < 'a > , namespace : & 'a str , name : & 'a str , member : Item < 'a >) { members . entry (namespace) . or_default () . entry (name) . or_default () . push (member) ; }
};
}
