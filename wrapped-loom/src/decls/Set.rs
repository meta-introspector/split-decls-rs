macro_rules! deps {
    () => {
        Thread!();
        VersionVec!();
        Id!();
    };
}

macro_rules! Set {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct Set { # [doc = " Unique execution identifier"] execution_id : execution :: Id , # [doc = " Set of threads"] threads : Vec < Thread > , # [doc = " Currently scheduled thread."] # [doc = ""] # [doc = " `None` signifies that no thread is runnable."] active : Option < usize > , # [doc = " Sequential consistency causality. All sequentially consistent operations"] # [doc = " synchronize with this causality."] pub seq_cst_causality : VersionVec , # [doc = " `tracing` span used as the parent for new thread spans."] iteration_span : tracing :: Span , }
    };
}

Set!();