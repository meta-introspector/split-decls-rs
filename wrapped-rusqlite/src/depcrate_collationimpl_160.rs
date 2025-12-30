// Generated macro for impl_160 (impl)
macro_rules! Depcrate_collationimpl_160 {
() => {
// Module: crate::collation
// Provides: {"impl_160"}
// Dependencies: {}
impl Connection { # [doc = " Add or modify a collation."] # [inline] pub fn create_collation < C , N : Name > (& self , collation_name : N , x_compare : C) -> Result < () > where C : Fn (& str , & str) -> Ordering + Send + 'static , { self . db . borrow_mut () . create_collation (collation_name , x_compare) } # [doc = " Collation needed callback"] # [inline] pub fn collation_needed (& self , x_coll_needed : fn (& Self , & str) -> Result < () >) -> Result < () > { self . db . borrow_mut () . collation_needed (x_coll_needed) } # [doc = " Remove collation."] # [inline] pub fn remove_collation < N : Name > (& self , collation_name : N) -> Result < () > { self . db . borrow_mut () . remove_collation (collation_name) } }
};
}
