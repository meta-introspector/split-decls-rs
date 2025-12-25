use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
mod tests {
    #[test]
    fn context_is_send() {
        fn assert_is_send<T: Send>() {}
        assert_is_send::<crate::Context<gimli::read::EndianSlice<'_, gimli::LittleEndian>>>();
    }
}
