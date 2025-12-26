use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl_intern_lookup!(
    ExpandDatabase,
    MacroCallId,
    MacroCallLoc,
    intern_macro_call,
    lookup_intern_macro_call
);
