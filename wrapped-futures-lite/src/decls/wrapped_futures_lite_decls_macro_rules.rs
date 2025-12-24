use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Pins a variable of type `T` on the stack and rebinds it as `Pin<&mut T>`.
///
/// ```
/// use futures_lite::{future, pin};
/// use std::fmt::Debug;
/// use std::future::Future;
/// use std::pin::Pin;
/// use std::time::Instant;
///
/// // Inspects each invocation of `Future::poll()`.
/// async fn inspect<T: Debug>(f: impl Future<Output = T>) -> T {
///     pin!(f);
///     future::poll_fn(|cx| dbg!(f.as_mut().poll(cx))).await
/// }
///
/// # spin_on::spin_on(async {
/// let f = async { 1 + 2 };
/// inspect(f).await;
/// # })
/// ```
#[macro_export]
macro_rules! pin {
    ($($x:ident),* $(,)?) => {
        $(let mut $x = ::core::pin::pin!($x);)*
    };
}
