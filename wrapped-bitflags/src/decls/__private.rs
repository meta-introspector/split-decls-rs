macro_rules! __private {
    () => {
        # [doc (hidden)] pub mod __private { # [allow (unused_imports)] pub use crate :: { external :: __private :: * , traits :: __private :: * } ; pub use core ; }
    };
}

__private!()