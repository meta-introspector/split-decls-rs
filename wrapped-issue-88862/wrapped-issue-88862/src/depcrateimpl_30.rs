// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl < Func , A , Res > N < (A ,) , Res > for Func where Func : Fn (A) -> Res + 'static , Res : std :: future :: Future , Res :: Output : Responder , { fn call (& self , _ : (A ,)) -> Res { todo ! () } }
};
}
