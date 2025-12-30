// Generated macro for impl_608 (impl)
macro_rules! Depcrate_process_results_implimpl_608 {
() => {
// Module: crate::process_results_impl
// Provides: {"impl_608"}
// Dependencies: {}
impl < I , T , E > DoubleEndedIterator for ProcessResults < '_ , I , E > where I : Iterator < Item = Result < T , E > > , I : DoubleEndedIterator , { fn next_back (& mut self) -> Option < Self :: Item > { let item = self . iter . next_back () ; self . next_body (item) } fn rfold < B , F > (mut self , init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { let error = self . error ; self . iter . try_rfold (init , | acc , opt | match opt { Ok (x) => Ok (f (acc , x)) , Err (e) => { * error = Err (e) ; Err (acc) } }) . unwrap_or_else (| e | e) } }
};
}
