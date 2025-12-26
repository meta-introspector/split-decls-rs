use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Simplified version of the signals stream.
///
/// This one simply returns the signal numbers, while [`SignalsInfo`] can provide additional
/// information.
pub type Signals = SignalsInfo<SignalOnly>;
