use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Formats arguments to a [`SmolStr`], potentially without allocating.
///
/// See [`alloc::format!`] or [`format_args!`] for syntax documentation.
#[macro_export]
macro_rules! format_smolstr {
    ($($tt:tt)*) => {
        { let mut w = $crate::SmolStrBuilder::new(); ::core::fmt::Write::write_fmt(& mut
        w, format_args!($($tt)*))
        .expect("a formatting trait implementation returned an error"); w.finish() }
    };
}
