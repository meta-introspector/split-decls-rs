use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Specialized [`FluentBundle`](crate::bundle::FluentBundle) over
/// non-concurrent [`IntlLangMemoizer`](intl_memoizer::IntlLangMemoizer).
///
/// This is the basic variant of the [`FluentBundle`](crate::bundle::FluentBundle).
///
/// The concurrent specialization can be constructed with
/// [`FluentBundle::new_concurrent`](crate::concurrent::FluentBundle::new_concurrent).
pub type FluentBundle<R> = bundle::FluentBundle<R, intl_memoizer::IntlLangMemoizer>;
