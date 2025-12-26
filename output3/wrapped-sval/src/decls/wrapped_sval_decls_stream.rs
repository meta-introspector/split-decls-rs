use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/**
Stream a value through a stream.
*/
pub fn stream<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl Value + ?Sized),
) -> Result {
    stream.value(value)
}
