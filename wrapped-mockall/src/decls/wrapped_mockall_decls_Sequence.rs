use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Used to enforce that mock calls must happen in the sequence specified.
///
/// Each expectation must expect to be called a fixed number of times.  Once
/// satisfied, the next expectation in the sequence will expect to be called.
///
/// # Examples
/// ```
/// # use mockall::*;
/// #[automock]
/// trait Foo {
///     fn foo(&self);
///     fn bar(&self) -> u32;
/// }
/// let mut seq = Sequence::new();
///
/// let mut mock0 = MockFoo::new();
/// let mut mock1 = MockFoo::new();
///
/// mock0.expect_foo()
///     .times(1)
///     .returning(|| ())
///     .in_sequence(&mut seq);
///
/// mock1.expect_bar()
///     .times(1)
///     .returning(|| 42)
///     .in_sequence(&mut seq);
///
/// mock0.foo();
/// mock1.bar();
/// ```
///
/// It is an error to add an expectation to a `Sequence` if its call count is
/// unspecified.
/// ```should_panic(expected = "with an exact call count")
/// # use mockall::*;
/// #[automock]
/// trait Foo {
///     fn foo(&self);
/// }
/// let mut seq = Sequence::new();
///
/// let mut mock = MockFoo::new();
/// mock.expect_foo()
///     .returning(|| ())
///     .in_sequence(&mut seq);  // panics!
/// ```
#[derive(Default)]
pub struct Sequence {
    inner: Arc<SeqInner>,
    /// Counter to use for the next SeqHandle associated with this Sequence
    next_seq: usize,
}
