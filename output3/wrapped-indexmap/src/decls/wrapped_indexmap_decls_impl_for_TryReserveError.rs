use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl TryReserveError {
    fn from_alloc(error: alloc::collections::TryReserveError) -> Self {
        Self {
            kind: TryReserveErrorKind::Std(error),
        }
    }
    fn from_hashbrown(error: hashbrown::TryReserveError) -> Self {
        Self {
            kind: match error {
                hashbrown::TryReserveError::CapacityOverflow => {
                    TryReserveErrorKind::CapacityOverflow
                }
                hashbrown::TryReserveError::AllocError { layout } => {
                    TryReserveErrorKind::AllocError { layout }
                }
            },
        }
    }
}
