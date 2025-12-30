// Generated macro for State (enum)
macro_rules! Depcrate_dereferenceState {
() => {
// Module: crate::dereference
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug)] enum State { DerefMethod { ty_changed_count : usize , is_ufcs : bool , # [doc = " The required mutability"] mutbl : Mutability , } , DerefedBorrow (DerefedBorrow) , ExplicitDeref { mutability : Option < Mutability > , } , ExplicitDerefField { name : Symbol , derefs_manually_drop : bool , } , Reborrow { mutability : Mutability , } , Borrow { mutability : Mutability , } , }
};
}
