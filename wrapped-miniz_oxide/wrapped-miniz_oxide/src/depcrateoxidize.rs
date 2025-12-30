// Generated macro for oxidize (macro)
macro_rules! Depcrateoxidize {
() => {
// Module: crate
// Provides: {"oxidize"}
// Dependencies: {}
macro_rules ! oxidize { ($ mz_func : ident , $ mz_func_oxide : ident ; $ ($ arg_name : ident : $ type_name : ident) ,*) => { unmangle ! (pub unsafe extern "C" fn $ mz_func (stream : * mut mz_stream , $ ($ arg_name : $ type_name) ,*) -> c_int { match stream . as_mut () { None => MZError :: Stream as c_int , Some (stream) => { match catch_unwind (AssertUnwindSafe (|| { match StreamOxide :: try_new (stream) { Ok (mut stream_oxide) => { let status = $ mz_func_oxide (& mut stream_oxide , $ ($ arg_name) ,*) ; * stream = stream_oxide . into_mz_stream () ; as_c_return_code (status) } Err (e) => { e as c_int } } })) { Ok (res) => res , Err (_) => { println ! ("FATAL ERROR: Caught panic!") ; MZError :: Stream as c_int } , } } } }) ; } ; }
};
}
