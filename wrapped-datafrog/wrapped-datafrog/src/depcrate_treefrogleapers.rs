// Generated macro for Leapers (trait)
macro_rules! Depcrate_treefrogLeapers {
() => {
// Module: crate::treefrog
// Provides: {"Leapers"}
// Dependencies: {}
# [doc = " Implemented for a tuple of leapers"] pub trait Leapers < 'leap , Tuple , Val > { # [doc = " Internal method:"] fn for_each_count (& mut self , tuple : & Tuple , op : impl FnMut (usize , usize)) ; # [doc = " Internal method:"] fn propose (& mut self , tuple : & Tuple , min_index : usize , values : & mut Vec < & 'leap Val >) ; # [doc = " Internal method:"] fn intersect (& mut self , tuple : & Tuple , min_index : usize , values : & mut Vec < & 'leap Val >) ; }
};
}
