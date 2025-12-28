macro_rules! deps {
    () => {
        Context!();
        Result!();
        SqlFnOutput!();
    };
}

macro_rules! Aggregate {
    () => {
        deps!();
        # [doc = " Aggregate is the callback interface for user-defined"] # [doc = " aggregate function."] # [doc = ""] # [doc = " `A` is the type of the aggregation context and `T` is the type of the final"] # [doc = " result. Implementations should be stateless."] pub trait Aggregate < A , T > where A : RefUnwindSafe + UnwindSafe , T : SqlFnOutput , { # [doc = " Initializes the aggregation context. Will be called prior to the first"] # [doc = " call to [`step()`](Aggregate::step) to set up the context for an"] # [doc = " invocation of the function. (Note: `init()` will not be called if"] # [doc = " there are no rows.)"] fn init (& self , ctx : & mut Context < '_ >) -> Result < A > ; # [doc = " \"step\" function called once for each row in an aggregate group. May be"] # [doc = " called 0 times if there are no rows."] fn step (& self , ctx : & mut Context < '_ > , acc : & mut A) -> Result < () > ; # [doc = " Computes and returns the final result. Will be called exactly once for"] # [doc = " each invocation of the function. If [`step()`](Aggregate::step) was"] # [doc = " called at least once, will be given `Some(A)` (the same `A` as was"] # [doc = " created by [`init`](Aggregate::init) and given to"] # [doc = " [`step`](Aggregate::step)); if [`step()`](Aggregate::step) was not"] # [doc = " called (because the function is running against 0 rows), will be"] # [doc = " given `None`."] # [doc = ""] # [doc = " The passed context will have no arguments."] fn finalize (& self , ctx : & mut Context < '_ > , acc : Option < A >) -> Result < T > ; }
    };
}

Aggregate!();