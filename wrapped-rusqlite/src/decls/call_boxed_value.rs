macro_rules! deps {
    () => {
        SqlFnOutput!();
        Error!();
        WindowAggregate!();
    };
}

macro_rules! call_boxed_value {
    () => {
        deps!();
        # [cfg (feature = "window")] unsafe extern "C" fn call_boxed_value < A , W , T > (ctx : * mut sqlite3_context) where A : RefUnwindSafe + UnwindSafe , W : WindowAggregate < A , T > , T : SqlFnOutput , { let pac = aggregate_context (ctx , 0) . filter (| & pac | { # [expect (clippy :: unnecessary_cast)] ! (* pac as * mut A) . is_null () }) ; let r = catch_unwind (| | { let boxed_aggr : * mut W = ffi :: sqlite3_user_data (ctx) . cast :: < W > () ; assert ! (! boxed_aggr . is_null () , "Internal error - null aggregate pointer") ; (* boxed_aggr) . value (pac . map (| pac | & mut * * pac)) }) ; let t = match r { Err (_) => { report_error (ctx , & Error :: UnwindingPanic) ; return ; } Ok (r) => r , } ; sql_result (ctx , & [] , t) ; }
    };
}

call_boxed_value!()