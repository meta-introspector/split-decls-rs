use serde::{Deserialize, Serialize};
use std::collections::HashMap;
thread_local! {
    static OUT : RefCell < Option < Sender < Message >>> = const { RefCell::new(None) };
    static TID : RefCell < Option < usize >> = const { RefCell::new(None) };
}
