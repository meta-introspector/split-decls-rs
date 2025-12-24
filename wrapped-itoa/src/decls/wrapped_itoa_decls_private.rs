use serde::{Deserialize, Serialize};
use std::collections::HashMap;
mod private {
    #[doc(hidden)]
    pub trait Sealed: Copy {
        #[doc(hidden)]
        type Buffer: 'static;
        fn write(self, buf: &mut Self::Buffer) -> &str;
    }
}
