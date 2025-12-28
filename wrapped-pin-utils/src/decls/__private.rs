macro_rules! __private {
    () => {
        # [doc (hidden)] pub mod __private { pub use core :: pin :: Pin ; }
    };
}

__private!();