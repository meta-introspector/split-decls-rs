use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Connection is just a pair of channels of LSP messages.
pub struct Connection {
    pub sender: Sender<Message>,
    pub receiver: Receiver<Message>,
}
