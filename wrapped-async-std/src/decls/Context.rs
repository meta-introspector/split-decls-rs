macro_rules! Context {
    () => {
        # [doc = " Add additional context to errors"] # [cfg (feature = "std")] pub (crate) trait Context { fn context (self , message : impl Fn () -> String) -> Self ; }
    };
}

Context!()