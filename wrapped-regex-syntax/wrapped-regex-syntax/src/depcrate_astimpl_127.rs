// Generated macro for impl_127 (impl)
macro_rules! Depcrate_astimpl_127 {
() => {
// Module: crate::ast
// Provides: {"impl_127"}
// Dependencies: {}
impl Flags { # [doc = " Add the given item to this sequence of flags."] # [doc = ""] # [doc = " If the item was added successfully, then `None` is returned. If the"] # [doc = " given item is a duplicate, then `Some(i)` is returned, where"] # [doc = " `items[i].kind == item.kind`."] pub fn add_item (& mut self , item : FlagsItem) -> Option < usize > { for (i , x) in self . items . iter () . enumerate () { if x . kind == item . kind { return Some (i) ; } } self . items . push (item) ; None } # [doc = " Returns the state of the given flag in this set."] # [doc = ""] # [doc = " If the given flag is in the set but is negated, then `Some(false)` is"] # [doc = " returned."] # [doc = ""] # [doc = " If the given flag is in the set and is not negated, then `Some(true)`"] # [doc = " is returned."] # [doc = ""] # [doc = " Otherwise, `None` is returned."] pub fn flag_state (& self , flag : Flag) -> Option < bool > { let mut negated = false ; for x in & self . items { match x . kind { FlagsItemKind :: Negation => { negated = true ; } FlagsItemKind :: Flag (ref xflag) if xflag == & flag => { return Some (! negated) ; } _ => { } } } None } }
};
}
