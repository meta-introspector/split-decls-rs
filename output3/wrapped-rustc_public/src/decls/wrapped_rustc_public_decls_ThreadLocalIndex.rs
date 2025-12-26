use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[expect(non_upper_case_globals)]
/// Emulating unit struct `struct ThreadLocalIndex`;
pub const ThreadLocalIndex: ThreadLocalIndex = ThreadLocalIndex {
    _phantom: PhantomData,
};
