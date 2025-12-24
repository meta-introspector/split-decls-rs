use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Encode the given value into a custom [Writer].
///
/// See the [config] module for more information on configurations.
///
/// [config]: config/index.html
pub fn encode_into_writer<E: enc::Encode, W: Writer, C: Config>(
    val: E,
    writer: W,
    config: C,
) -> Result<(), error::EncodeError> {
    let mut encoder = enc::EncoderImpl::<_, C>::new(writer, config);
    val.encode(&mut encoder)?;
    Ok(())
}
