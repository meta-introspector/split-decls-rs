macro_rules! __private {
    () => {
        # [doc (hidden)] pub mod __private { # [cfg (feature = "getrandom")] pub use getrandom ; # [cfg (feature = "rand")] pub use rand ; }
    };
}

__private!();