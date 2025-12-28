macro_rules! deps {
    () => {
        Error!();
        WindowAggregate!();
        SqlFnOutput!();
        Context!();
    };
}

macro_rules! call_boxed_inverse {
    () => {
        deps!();
        # [cfg (feature = "window")] unsafe extern "C" fn call_boxed_inverse < A , W , T > (ctx : * mut sqlite3_context , argc : c_int , argv : * mut * mut sqlite3_value ,) where A : RefUnwindSafe + UnwindSafe , W : WindowAggregate < A , T > , T : SqlFnOutput , { let Some (pac) = aggregate_context (ctx , size_of :: < * mut A > ()) else { ffi :: sqlite3_result_error_nomem (ctx) ; return ; } ; let r = catch_unwind (| | { let boxed_aggr : * mut W = ffi :: sqlite3_user_data (ctx) . cast :: < W > () ; assert ! (! boxed_aggr . is_null () , "Internal error - null aggregate pointer") ; let mut ctx = Context { ctx , args : slice :: from_raw_parts (argv , argc as usize) , } ; (* boxed_aggr) . inverse (& mut ctx , & mut * * pac) }) ; let r = match r { Err (_) => { report_error (ctx , & Error :: UnwindingPanic) ; return ; } Ok (r) => r , } ; match r { Ok (_) => { } Err (err) => report_error (ctx , & err) , } ; }
    };
}

call_boxed_inverse!();