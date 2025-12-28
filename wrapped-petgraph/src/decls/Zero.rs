macro_rules! Zero {
    () => {
        # [doc = " Base trait for types that can be wrapped in a [`NotZero`](struct.NotZero.html)."] # [doc = ""] # [doc = " Implementors must provide a singleton object that will be used to mark empty edges in a"] # [doc = " [`MatrixGraph`](struct.MatrixGraph.html)."] # [doc = ""] # [doc = " Note that this trait is already implemented for the base numeric types."] pub trait Zero { # [doc = " Return the singleton object which can be used as a sentinel value."] fn zero () -> Self ; # [doc = " Return true if `self` is equal to the sentinel value."] fn is_zero (& self) -> bool ; }
    };
}

Zero!();