// Generated macro for call_boxed_final (function)
macro_rules! Depcrate_functionscall_boxed_final {
() => {
// Module: crate::functions
// Provides: {"call_boxed_final"}
// Dependencies: {}
unsafe extern "C" fn call_boxed_final < A , D , T > (ctx : * mut sqlite3_context) where A : RefUnwindSafe + UnwindSafe , D : Aggregate < A , T > , T : SqlFnOutput , { let a : Option < A > = match aggregate_context (ctx , 0) { Some (pac) => { # [expect (clippy :: unnecessary_cast)] if (* pac as * mut A) . is_null () { None } else { let a = Box :: from_raw (* pac) ; Some (* a) } } None => None , } ; let r = catch_unwind (| | { let boxed_aggr : * mut D = ffi :: sqlite3_user_data (ctx) . cast :: < D > () ; assert ! (! boxed_aggr . is_null () , "Internal error - null aggregate pointer") ; let mut ctx = Context { ctx , args : & mut [] } ; (* boxed_aggr) . finalize (& mut ctx , a) }) ; let t = match r { Err (_) => { report_error (ctx , & Error :: UnwindingPanic) ; return ; } Ok (r) => r , } ; sql_result (ctx , & [] , t) ; }
};
}
