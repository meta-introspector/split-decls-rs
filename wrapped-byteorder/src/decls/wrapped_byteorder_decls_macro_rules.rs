use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Copies a &[$ty] $src into a &mut [u8] $dst for the endianness given by
/// $from_bytes (must be either from_be_bytes or from_le_bytes).
///
/// Panics if $src.len() * size_of::<$ty>() != $dst.len().
macro_rules! write_slice {
    ($src:expr, $dst:expr, $ty:ty, $to_bytes:ident) => {
        { const SIZE : usize = core::mem::size_of::<$ty > (); let src : & [$ty] = $src;
        let dst : & mut [u8] = $dst; assert_eq!(src.len() * SIZE, dst.len()); for (src,
        dst) in src.iter().zip(dst.chunks_exact_mut(SIZE)) { dst.copy_from_slice(& src
        .$to_bytes ()); } }
    };
}
