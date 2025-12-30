// Generated macro for impl_1217 (impl)
macro_rules! Depcrate_iter_unzipimpl_1217 {
() => {
// Module: crate::iter::unzip
// Provides: {"impl_1217"}
// Dependencies: {}
impl < A , B , RA , RB > Reducer < (A , B) > for UnzipReducer < RA , RB > where RA : Reducer < A > , RB : Reducer < B > , { fn reduce (self , left : (A , B) , right : (A , B)) -> (A , B) { (self . left . reduce (left . 0 , right . 0) , self . right . reduce (left . 1 , right . 1) ,) } }
};
}
