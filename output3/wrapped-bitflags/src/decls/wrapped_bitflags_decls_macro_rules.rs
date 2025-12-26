use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Implement a flag, which may be a wildcard `_`.
#[macro_export]
#[doc(hidden)]
macro_rules! __bitflags_flag {
    ({ name : _, named : { $($named:tt)* }, unnamed : { $($unnamed:tt)* }, }) => {
        $($unnamed)*
    };
    (
        { name : $Flag:ident, named : { $($named:tt)* }, unnamed : { $($unnamed:tt)* }, }
    ) => {
        $($named)*
    };
}
