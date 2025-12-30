// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl FooDb { fn acquire (& self) -> impl std :: future :: Future < Output = Result < G , Error > > + 'static { let _shared : std :: sync :: Arc < SharedPool > = todo ! () ; async move { _shared . acquire () . await . map (| conn | conn . attach (& _shared)) } } async fn nested (& self) -> Result < Option < String > , () > { (async move { match async move { async move { (async move { match async move { let db = FooDb ; let mut _conn = db . acquire () . await . unwrap () ; Ok :: < _ , () > (String :: default ()) } . await { Ok (x) => Ok (x) , Err0 => todo ! () , } } , todo ! () ,) . 0 . await } . await . map (Some) } . await { Ok (x) => Ok (x) , Err (e) => Err (e) , } } ,) . 0 . await } }
};
}
