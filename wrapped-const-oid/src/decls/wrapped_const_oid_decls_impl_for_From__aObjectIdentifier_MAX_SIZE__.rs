use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a, const MAX_SIZE: usize> From<&'a ObjectIdentifier<MAX_SIZE>>
for &'a ObjectIdentifierRef {
    fn from(oid: &'a ObjectIdentifier<MAX_SIZE>) -> &'a ObjectIdentifierRef {
        oid.as_oid_ref()
    }
}
