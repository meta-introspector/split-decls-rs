// Generated macro for impl_250 (impl)
macro_rules! Depcrate_treeimpl_250 {
() => {
// Module: crate::tree
// Provides: {"impl_250"}
// Dependencies: {}
impl Tree < Item > { # [doc = " Truncates the preceding siblings to the given end position,"] # [doc = " and returns the new current node."] pub (crate) fn truncate_siblings (& mut self , end_byte_ix : usize) { let parent_ix = self . peek_up () . unwrap () ; let mut next_child_ix = self [parent_ix] . child ; let mut prev_child_ix = None ; while let Some (child_ix) = next_child_ix { let child_end = self [child_ix] . item . end ; if child_end < end_byte_ix { prev_child_ix = Some (child_ix) ; next_child_ix = self [child_ix] . next ; continue ; } else if child_end == end_byte_ix { self [child_ix] . next = None ; self . cur = Some (child_ix) ; } else if self [child_ix] . item . start == end_byte_ix { let is_previous_char_backslash_escape = match self [child_ix] . item . body { ItemBody :: Text { backslash_escaped } => backslash_escaped , _ => false , } ; if is_previous_char_backslash_escape { let last_byte_ix = end_byte_ix - 1 ; self [child_ix] . item . start = last_byte_ix ; self [child_ix] . item . end = end_byte_ix ; self . cur = Some (child_ix) ; } else if let Some (prev_child_ix) = prev_child_ix { self [prev_child_ix] . next = None ; self . cur = Some (prev_child_ix) ; } else { self [parent_ix] . child = None ; self . cur = None ; } } else { debug_assert ! (self [child_ix] . item . start < end_byte_ix) ; debug_assert ! (end_byte_ix < child_end) ; self [child_ix] . item . end = end_byte_ix ; self [child_ix] . next = None ; self . cur = Some (child_ix) ; } break ; } } }
};
}
