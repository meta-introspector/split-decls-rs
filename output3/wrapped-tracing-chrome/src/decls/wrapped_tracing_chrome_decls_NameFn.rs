use serde::{Deserialize, Serialize};
use std::collections::HashMap;
type NameFn<S> = Box<dyn Fn(&EventOrSpan<'_, '_, S>) -> String + Send + Sync>;
