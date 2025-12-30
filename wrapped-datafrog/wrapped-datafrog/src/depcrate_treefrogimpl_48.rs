// Generated macro for impl_48 (impl)
macro_rules! Depcrate_treefrogimpl_48 {
() => {
// Module: crate::treefrog
// Provides: {"impl_48"}
// Dependencies: {}
impl < Key : Ord , Val : Ord > RelationLeaper < Key , Val > for Relation < (Key , Val) > { fn extend_with < 'leap , Tuple : Ord , Func : Fn (& Tuple) -> Key > (& 'leap self , key_func : Func ,) -> extend_with :: ExtendWith < 'leap , Key , Val , Tuple , Func > where Key : 'leap , Val : 'leap , { extend_with :: ExtendWith :: from (self , key_func) } fn extend_anti < 'leap , Tuple : Ord , Func : Fn (& Tuple) -> Key > (& 'leap self , key_func : Func ,) -> extend_anti :: ExtendAnti < 'leap , Key , Val , Tuple , Func > where Key : 'leap , Val : 'leap , { extend_anti :: ExtendAnti :: from (self , key_func) } fn filter_with < 'leap , Tuple : Ord , Func : Fn (& Tuple) -> (Key , Val) > (& 'leap self , key_func : Func ,) -> filter_with :: FilterWith < 'leap , Key , Val , Tuple , Func > where Key : 'leap , Val : 'leap , { filter_with :: FilterWith :: from (self , key_func) } fn filter_anti < 'leap , Tuple : Ord , Func : Fn (& Tuple) -> (Key , Val) > (& 'leap self , key_func : Func ,) -> filter_anti :: FilterAnti < 'leap , Key , Val , Tuple , Func > where Key : 'leap , Val : 'leap , { filter_anti :: FilterAnti :: from (self , key_func) } }
};
}
