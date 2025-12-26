use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a> Deref for MaybeUninitSlice<'a> {
    type Target = [MaybeUninit<u8>];
    fn deref(&self) -> &[MaybeUninit<u8>] {
        self.0.as_slice()
    }
}
