// Generated macro for impl_2042 (impl)
macro_rules! Depcrate_compat_executorimpl_2042 {
() => {
// Module: crate::compat::executor
// Provides: {"impl_2042"}
// Dependencies: {}
impl < Sp , Fut > Executor01 < Fut > for Compat < Sp > where for < 'a > & 'a Sp : Spawn03 , Fut : Future01 < Item = () , Error = () > + Send + 'static , { fn execute (& self , future : Fut) -> Result < () , ExecuteError01 < Fut > > { (& self . inner) . spawn (future . compat () . map (| _ | ())) . expect ("unable to spawn future from Compat executor") ; Ok (()) } }
};
}
