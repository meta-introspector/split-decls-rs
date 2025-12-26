use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Asserts that a type is `True`, aka `B1`.
#[macro_export]
macro_rules! assert_type {
    ($a:ty) => {
        const _: core::marker::PhantomData<<$a as $crate::Same<True>>::Output> =
            core::marker::PhantomData;
    };
}
