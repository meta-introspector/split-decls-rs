use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod prelude {
    //! A "prelude" for crates using the `futures` crate.
    //!
    //! This prelude is similar to the standard library's prelude in that you'll
    //! almost always want to import its entire contents, but unlike the
    //! standard library's prelude you'll have to do so manually:
    //!
    //! ```
    //! # #[allow(unused_imports)]
    //! use futures::prelude::*;
    //! ```
    //!
    //! The prelude may grow over time as additional items see ubiquitous use.
    pub use crate::future::{self, Future, TryFuture};
    #[doc(no_inline)]
    pub use crate::future::{FutureExt as _, TryFutureExt as _};
    #[cfg(feature = "std")]
    pub use crate::io::{AsyncBufRead, AsyncRead, AsyncSeek, AsyncWrite};
    #[cfg(feature = "std")]
    #[doc(no_inline)]
    pub use crate::io::{
        AsyncBufReadExt as _, AsyncReadExt as _, AsyncSeekExt as _, AsyncWriteExt as _,
    };
    #[doc(no_inline)]
    pub use crate::sink::SinkExt as _;
    pub use crate::sink::{self, Sink};
    pub use crate::stream::{self, Stream, TryStream};
    #[doc(no_inline)]
    pub use crate::stream::{StreamExt as _, TryStreamExt as _};
}
