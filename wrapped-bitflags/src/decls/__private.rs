macro_rules! __private {
    () => {
        pub (crate) mod __private { # [cfg (feature = "serde")] pub use serde_core as serde ; # [cfg (feature = "arbitrary")] pub use arbitrary ; # [cfg (feature = "bytemuck")] pub use bytemuck ; }
    };
}

__private!()