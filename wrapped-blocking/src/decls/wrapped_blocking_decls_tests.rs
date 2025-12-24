use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    #[test]
    fn test_max_threads() {
        env::set_var(MAX_THREADS_ENV, "100");
        assert_eq!(100, Executor::max_threads().get());
        env::set_var(MAX_THREADS_ENV, "0");
        assert_eq!(1, Executor::max_threads().get());
        env::set_var(MAX_THREADS_ENV, "50000");
        assert_eq!(10000, Executor::max_threads().get());
        env::set_var(MAX_THREADS_ENV, "");
        assert_eq!(500, Executor::max_threads().get());
        env::set_var(MAX_THREADS_ENV, "NOTINT");
        assert_eq!(500, Executor::max_threads().get());
    }
}
