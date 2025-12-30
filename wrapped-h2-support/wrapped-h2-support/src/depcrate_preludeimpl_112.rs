// Generated macro for impl_112 (impl)
macro_rules! Depcrate_preludeimpl_112 {
() => {
// Module: crate::prelude
// Provides: {"impl_112"}
// Dependencies: {}
impl < T , B > ClientExt for client :: Connection < T , B > where T : AsyncRead + AsyncWrite + Unpin + 'static , B : Buf , { fn run < 'a , F : Future + Unpin + 'a > (& 'a mut self , f : F ,) -> Pin < Box < dyn Future < Output = F :: Output > + 'a > > { let res = future :: select (self , f) ; Box :: pin (async { match res . await { Left ((Ok (_) , b)) => { b . await } Right ((v , _)) => v , Left ((Err (e) , _)) => panic ! ("err: {:?}" , e) , } }) } }
};
}
