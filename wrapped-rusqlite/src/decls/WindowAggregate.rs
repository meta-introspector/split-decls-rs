macro_rules! deps {
    () => {
        Result!();
        Aggregate!();
        SqlFnOutput!();
        Context!();
    };
}

macro_rules! WindowAggregate {
    () => {
        deps!();
        # [doc = " `WindowAggregate` is the callback interface for"] # [doc = " user-defined aggregate window function."] # [cfg (feature = "window")] pub trait WindowAggregate < A , T > : Aggregate < A , T > where A : RefUnwindSafe + UnwindSafe , T : SqlFnOutput , { # [doc = " Returns the current value of the aggregate. Unlike xFinal, the"] # [doc = " implementation should not delete any context."] fn value (& self , acc : Option < & mut A >) -> Result < T > ; # [doc = " Removes a row from the current window."] fn inverse (& self , ctx : & mut Context < '_ > , acc : & mut A) -> Result < () > ; }
    };
}

WindowAggregate!()