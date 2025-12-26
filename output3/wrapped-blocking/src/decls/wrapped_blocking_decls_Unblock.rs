use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Runs blocking I/O on a thread pool.
///
/// Blocking I/O must be isolated from async code. This type moves blocking I/O operations onto a
/// special thread pool while exposing a familiar async interface.
///
/// This type implements traits [`Stream`], [`AsyncRead`], [`AsyncWrite`], or [`AsyncSeek`] if the
/// inner type implements [`Iterator`], [`Read`], [`Write`], or [`Seek`], respectively.
///
/// # Caveats
///
/// [`Unblock`] is a low-level primitive, and as such it comes with some caveats.
///
/// For higher-level primitives built on top of [`Unblock`], look into [`async-fs`] or
/// [`async-process`] (on Windows).
///
/// [`async-fs`]: https://github.com/smol-rs/async-fs
/// [`async-process`]: https://github.com/smol-rs/async-process
///
/// [`Unblock`] communicates with I/O operations on the thread pool through a pipe. That means an
/// async read/write operation simply receives/sends some bytes from/into the pipe. When in reading
/// mode, the thread pool reads bytes from the I/O handle and forwards them into the pipe until it
/// becomes full. When in writing mode, the thread pool reads bytes from the pipe and forwards them
/// into the I/O handle.
///
/// Use [`Unblock::with_capacity()`] to configure the capacity of the pipe.
///
/// ### Reading
///
/// If you create an [`Unblock`]`<`[`Stdin`][`std::io::Stdin`]`>`, read some bytes from it,
/// and then drop it, a blocked read operation may keep hanging on the thread pool. The next
/// attempt to read from stdin will lose bytes read by the hanging operation. This is a difficult
/// problem to solve, so make sure you only use a single stdin handle for the duration of the
/// entire program.
///
/// ### Writing
///
/// If writing data through the [`AsyncWrite`] trait, make sure to flush before dropping the
/// [`Unblock`] handle or some buffered data might get lost.
///
/// ### Seeking
///
/// Because of buffering in the pipe, if [`Unblock`] wraps a [`File`][`std::fs::File`], a single
/// read operation may move the file cursor farther than is the span of the operation. In fact,
/// reading just keeps going in the background until the pipe gets full. Keep this mind when
/// using [`AsyncSeek`] with [relative][`SeekFrom::Current`] offsets.
///
/// # Examples
///
/// ```
/// use blocking::Unblock;
/// use futures_lite::prelude::*;
///
/// # futures_lite::future::block_on(async {
/// let mut stdout = Unblock::new(std::io::stdout());
/// stdout.write_all(b"Hello world!").await?;
/// stdout.flush().await?;
/// # std::io::Result::Ok(()) });
/// ```
pub struct Unblock<T> {
    state: State<T>,
    cap: Option<usize>,
}
