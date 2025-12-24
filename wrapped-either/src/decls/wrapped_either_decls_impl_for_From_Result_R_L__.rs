use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Convert from `Result` to `Either` with `Ok => Right` and `Err => Left`.
impl<L, R> From<Result<R, L>> for Either<L, R> {
    fn from(r: Result<R, L>) -> Self {
        match r {
            Err(e) => Left(e),
            Ok(o) => Right(o),
        }
    }
}
