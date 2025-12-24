use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
enum ErrorKind {
    GlobalPoolAlreadyInitialized,
    CurrentThreadAlreadyInPool,
    IOError(io::Error),
}
