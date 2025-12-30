// Generated macro for tuple_leapers (macro)
macro_rules! Depcrate_treefrogtuple_leapers {
() => {
// Module: crate::treefrog
// Provides: {"tuple_leapers"}
// Dependencies: {}
macro_rules ! tuple_leapers { ($ ($ Ty : ident) *) => { # [allow (unused_assignments , non_snake_case)] impl <'leap , Tuple , Val , $ ($ Ty) ,*> Leapers <'leap , Tuple , Val > for ($ ($ Ty ,) *) where $ ($ Ty : Leaper <'leap , Tuple , Val >,) * { fn for_each_count (& mut self , tuple : & Tuple , mut op : impl FnMut (usize , usize)) { let ($ ($ Ty ,) *) = self ; let mut index = 0 ; $ (let count = $ Ty . count (tuple) ; op (index , count) ; index += 1 ;) * } fn propose (& mut self , tuple : & Tuple , min_index : usize , values : & mut Vec <&'leap Val >) { let ($ ($ Ty ,) *) = self ; let mut index = 0 ; $ (if min_index == index { return $ Ty . propose (tuple , values) ; } index += 1 ;) * panic ! ("no match found for min_index={}" , min_index) ; } fn intersect (& mut self , tuple : & Tuple , min_index : usize , values : & mut Vec <&'leap Val >) { let ($ ($ Ty ,) *) = self ; let mut index = 0 ; $ (if min_index != index { $ Ty . intersect (tuple , values) ; } index += 1 ;) * } } } }
};
}
