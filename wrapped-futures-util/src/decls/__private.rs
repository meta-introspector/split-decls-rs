macro_rules! __private {
    () => {
        # [doc (hidden)] pub mod __private { pub use crate :: * ; pub use core :: { option :: Option :: { self , None , Some } , pin :: Pin , result :: Result :: { Err , Ok } , } ; # [cfg (feature = "async-await")] pub mod async_await { pub use crate :: async_await :: * ; } }
    };
}

__private!()