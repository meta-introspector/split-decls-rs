use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Convert from `Either` to `Result` with `Right => Ok` and `Left => Err`.
impl<L, R> From<Either<L, R>> for Result<R, L> {
    fn from(val: Either<L, R>) -> Self {
        match val {
            Left(l) => Err(l),
            Right(r) => Ok(r),
        }
    }
}
