// Generated macro for UnresolvedName (enum)
macro_rules! Depcrate_astUnresolvedName {
() => {
// Module: crate::ast
// Provides: {"UnresolvedName"}
// Dependencies: {}
# [doc = " The `<unresolved-name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <unresolved-name> ::= [gs] <base-unresolved-name>"] # [doc = "                          #"] # [doc = "                   ::= sr <unresolved-type> <base-unresolved-name>"] # [doc = "                          #"] # [doc = "                   ::= srN <unresolved-type> <unresolved-qualifier-level>+ E <base-unresolved-name>"] # [doc = "                          #"] # [doc = "                   ::= [gs] sr <unresolved-qualifier-level>+ E <base-unresolved-name>"] # [doc = "                          # A::x, N::y, A<T>::z; \"gs\" means leading \"::\""] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum UnresolvedName { # [doc = " `x`"] Name (BaseUnresolvedName) , # [doc = " `::x`"] Global (BaseUnresolvedName) , # [doc = " `T::x`  or `decltype(p)::x` or `T::N::x` or `decltype(p)::N::x`"] Nested1 (UnresolvedTypeHandle , Vec < UnresolvedQualifierLevel > , BaseUnresolvedName ,) , # [doc = " `A::x` or `N::y` or `A<T>::z`"] Nested2 (Vec < UnresolvedQualifierLevel > , BaseUnresolvedName) , # [doc = " `::A::x` or `::N::y` or `::A<T>::z`"] GlobalNested2 (Vec < UnresolvedQualifierLevel > , BaseUnresolvedName) , }
};
}
