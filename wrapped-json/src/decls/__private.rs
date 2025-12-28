macro_rules! __private {
    () => {
        # [doc (hidden)] pub mod __private { # [doc (hidden)] pub use alloc :: vec ; }
    };
}

__private!()