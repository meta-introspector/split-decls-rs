use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[macro_export]
macro_rules! impl_intern_key {
    ($id:ident, $loc:ident) => {
        #[salsa_macros::interned(no_lifetime, revisions = usize::MAX)]
        #[derive(PartialOrd, Ord)] pub struct $id { pub loc : $loc, } impl
        ::std::fmt::Debug for $id { fn fmt(& self, f : & mut ::std::fmt::Formatter <'_ >)
        -> ::std::fmt::Result { f.debug_tuple(stringify!($id)).field(&
        format_args!("{:04x}", self.0.index())).finish() } }
    };
}
