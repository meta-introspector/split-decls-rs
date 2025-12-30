// Generated macro for define_id (macro)
macro_rules! Depcrate_writedefine_id {
() => {
// Module: crate::write
// Provides: {"define_id"}
// Dependencies: {}
macro_rules ! define_id { ($ name : ident , $ docs : expr) => { # [doc =$ docs] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct $ name { base_id : BaseId , index : usize , } impl $ name { # [inline] fn new (base_id : BaseId , index : usize) -> Self { $ name { base_id , index } } } } ; }
};
}
