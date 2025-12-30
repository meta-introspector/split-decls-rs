// Generated macro for Leaper (trait)
macro_rules! Depcrate_treefrogLeaper {
() => {
// Module: crate::treefrog
// Provides: {"Leaper"}
// Dependencies: {}
# [doc = " Methods to support treefrog leapjoin."] pub trait Leaper < 'leap , Tuple , Val > { # [doc = " Estimates the number of proposed values."] fn count (& mut self , prefix : & Tuple) -> usize ; # [doc = " Populates `values` with proposed values."] fn propose (& mut self , prefix : & Tuple , values : & mut Vec < & 'leap Val >) ; # [doc = " Restricts `values` to proposed values."] fn intersect (& mut self , prefix : & Tuple , values : & mut Vec < & 'leap Val >) ; }
};
}
