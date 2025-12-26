use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Multiple conflicting arbitrary attributes are used on the same field:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// struct Point {
///     #[arbitrary(value = 2)]
///     #[arbitrary(value = 2)]
///     x: i32,
/// }
/// ```
///
/// An unknown attribute:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// struct Point {
///     #[arbitrary(unknown_attr)]
///     x: i32,
/// }
/// ```
///
/// An unknown attribute with a value:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// struct Point {
///     #[arbitrary(unknown_attr = 13)]
///     x: i32,
/// }
/// ```
///
/// `value` without RHS:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// struct Point {
///     #[arbitrary(value)]
///     x: i32,
/// }
/// ```
///
/// `with` without RHS:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// struct Point {
///     #[arbitrary(with)]
///     x: i32,
/// }
/// ```
///
/// Multiple conflicting bounds at the container-level:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// #[arbitrary(bound = "T: Default")]
/// #[arbitrary(bound = "T: Default")]
/// struct Point<T: Default> {
///     #[arbitrary(default)]
///     x: T,
/// }
/// ```
///
/// Multiple conflicting bounds in a single bound attribute:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// #[arbitrary(bound = "T: Default, T: Default")]
/// struct Point<T: Default> {
///     #[arbitrary(default)]
///     x: T,
/// }
/// ```
///
/// Multiple conflicting bounds in multiple bound attributes:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// #[arbitrary(bound = "T: Default", bound = "T: Default")]
/// struct Point<T: Default> {
///     #[arbitrary(default)]
///     x: T,
/// }
/// ```
///
/// Too many bounds supplied:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// #[arbitrary(bound = "T: Default")]
/// struct Point {
///     x: i32,
/// }
/// ```
///
/// Too many bounds supplied across multiple attributes:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// #[arbitrary(bound = "T: Default")]
/// #[arbitrary(bound = "U: Default")]
/// struct Point<T: Default> {
///     #[arbitrary(default)]
///     x: T,
/// }
/// ```
///
/// Attempt to use the derive attribute on an enum variant:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// enum Enum<T: Default> {
///     #[arbitrary(default)]
///     Variant(T),
/// }
/// ```
#[cfg(all(doctest, feature = "derive"))]
pub struct CompileFailTests;
