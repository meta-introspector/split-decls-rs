// Generated macro for GlobalCtorDtor (enum)
macro_rules! Depcrate_astGlobalCtorDtor {
() => {
// Module: crate::ast
// Provides: {"GlobalCtorDtor"}
// Dependencies: {}
# [doc = " A global constructor or destructor."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum GlobalCtorDtor { # [doc = " A global constructor."] Ctor (Box < MangledName >) , # [doc = " A global destructor."] Dtor (Box < MangledName >) , }
};
}
