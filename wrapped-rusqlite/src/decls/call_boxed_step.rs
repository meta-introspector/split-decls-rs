macro_rules! deps {
    () => {
        Error!();
        SqlFnOutput!();
        Aggregate!();
        Context!();
    };
}

macro_rules! call_boxed_step {
    () => {
        deps!();
        unsafe extern "C" fn call_boxed_step < A , D , T > (ctx : * mut sqlite3_context , argc : c_int , argv : * mut * mut sqlite3_value ,) where A : RefUnwindSafe + UnwindSafe , D : Aggregate < A , T > , T : SqlFnOutput , { let Some (pac) = aggregate_context (ctx , size_of :: < * mut A > ()) else { ffi :: sqlite3_result_error_nomem (ctx) ; return ; } ; let r = catch_unwind (| | { let boxed_aggr : * mut D = ffi :: sqlite3_user_data (ctx) . cast :: < D > () ; assert ! (! boxed_aggr . is_null () , "Internal error - null aggregate pointer") ; let mut ctx = Context { ctx , args : slice :: from_raw_parts (argv , argc as usize) , } ; # [expect (clippy :: unnecessary_cast)] if (* pac as * mut A) . is_null () { * pac = Box :: into_raw (Box :: new ((* boxed_aggr) . init (& mut ctx) ?)) ; } (* boxed_aggr) . step (& mut ctx , & mut * * pac) }) ; let r = match r { Err (_) => { report_error (ctx , & Error :: UnwindingPanic) ; return ; } Ok (r) => r , } ; match r { Ok (_) => { } Err (err) => report_error (ctx , & err) , } ; }
    };
}

call_boxed_step!();