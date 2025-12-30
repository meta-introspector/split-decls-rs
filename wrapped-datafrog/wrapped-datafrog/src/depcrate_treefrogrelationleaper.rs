// Generated macro for RelationLeaper (trait)
macro_rules! Depcrate_treefrogRelationLeaper {
() => {
// Module: crate::treefrog
// Provides: {"RelationLeaper"}
// Dependencies: {}
# [doc = " Extension method for relations."] pub trait RelationLeaper < Key : Ord , Val : Ord > { # [doc = " Extend with `Val` using the elements of the relation."] fn extend_with < 'leap , Tuple : Ord , Func : Fn (& Tuple) -> Key > (& 'leap self , key_func : Func ,) -> extend_with :: ExtendWith < 'leap , Key , Val , Tuple , Func > where Key : 'leap , Val : 'leap ; # [doc = " Extend with `Val` using the complement of the relation."] fn extend_anti < 'leap , Tuple : Ord , Func : Fn (& Tuple) -> Key > (& 'leap self , key_func : Func ,) -> extend_anti :: ExtendAnti < 'leap , Key , Val , Tuple , Func > where Key : 'leap , Val : 'leap ; # [doc = " Extend with any value if tuple is present in relation."] fn filter_with < 'leap , Tuple : Ord , Func : Fn (& Tuple) -> (Key , Val) > (& 'leap self , key_func : Func ,) -> filter_with :: FilterWith < 'leap , Key , Val , Tuple , Func > where Key : 'leap , Val : 'leap ; # [doc = " Extend with any value if tuple is absent from relation."] fn filter_anti < 'leap , Tuple : Ord , Func : Fn (& Tuple) -> (Key , Val) > (& 'leap self , key_func : Func ,) -> filter_anti :: FilterAnti < 'leap , Key , Val , Tuple , Func > where Key : 'leap , Val : 'leap ; }
};
}
