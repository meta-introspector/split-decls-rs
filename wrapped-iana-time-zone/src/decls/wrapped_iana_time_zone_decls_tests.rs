use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_current() {
        println!("current: {}", get_timezone().unwrap());
    }
}
