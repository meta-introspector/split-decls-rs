macro_rules! LazyStatic {
    () => {
        # [doc = " Support trait for enabling a few common operations on lazy static values."] # [doc = ""] # [doc = " This is implemented by each defined lazy static, and"] # [doc = " used by the free functions in this crate."] pub trait LazyStatic { # [doc (hidden)] fn initialize (lazy : & Self) ; }
    };
}

LazyStatic!();