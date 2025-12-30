// Generated macro for tuple_try_from_row (macro)
macro_rules! Depcrate_rowtuple_try_from_row {
() => {
// Module: crate::row
// Provides: {"tuple_try_from_row"}
// Dependencies: {}
macro_rules ! tuple_try_from_row { ($ ($ field : ident) ,*) => { impl <'a , $ ($ field ,) *> convert :: TryFrom <&'a Row <'a >> for ($ ($ field ,) *) where $ ($ field : FromSql ,) * { type Error = crate :: Error ; # [allow (unused_assignments , unused_variables , unused_mut)] fn try_from (row : &'a Row <'a >) -> Result < Self > { let mut index = 0 ; $ (# [expect (non_snake_case)] let $ field = row . get ::< _ , $ field > (index) ?; index += 1 ;) * Ok (($ ($ field ,) *)) } } } }
};
}
