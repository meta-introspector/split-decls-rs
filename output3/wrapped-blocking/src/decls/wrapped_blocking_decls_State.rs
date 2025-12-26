use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Current state of a blocking task.
enum State<T> {
    /// There is no blocking task.
    ///
    /// The inner value is readily available, unless it has already been extracted. The value is
    /// extracted out by [`Unblock::into_inner()`], [`AsyncWrite::poll_close()`], or by awaiting
    /// [`Unblock`].
    Idle(Option<Box<T>>),
    /// A [`Unblock::with_mut()`] closure was spawned and is still running.
    WithMut(Task<Box<T>>),
    /// The inner value is an [`Iterator`] currently iterating in a task.
    ///
    /// The `dyn Any` value here is a `Pin<Box<Receiver<<T as Iterator>::Item>>>`.
    Streaming(Option<Box<dyn Any + Send + Sync>>, Task<Box<T>>),
    /// The inner value is a [`Read`] currently reading in a task.
    Reading(Option<Reader>, Task<(io::Result<()>, Box<T>)>),
    /// The inner value is a [`Write`] currently writing in a task.
    Writing(Option<Writer>, Task<(io::Result<()>, Box<T>)>),
    /// The inner value is a [`Seek`] currently seeking in a task.
    Seeking(Task<(SeekFrom, io::Result<u64>, Box<T>)>),
}
