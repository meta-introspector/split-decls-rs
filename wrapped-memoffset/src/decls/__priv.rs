macro_rules! __priv {
    () => {
        # [doc = " Hidden module for things the macros need to access."] # [doc (hidden)] pub mod __priv { # [doc (hidden)] pub use core :: mem ; # [doc (hidden)] pub use core :: ptr ; # [doc = " Use type inference to obtain the size of the pointee (without actually using the pointer)."] # [doc (hidden)] pub fn size_of_pointee < T > (_ptr : * const T) -> usize { mem :: size_of :: < T > () } }
    };
}

__priv!()