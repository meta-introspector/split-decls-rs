use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// IoUring instance
///
/// - `S`: The ring's submission queue entry (SQE) type, either [`squeue::Entry`] or
///   [`squeue::Entry128`];
/// - `C`: The ring's completion queue entry (CQE) type, either [`cqueue::Entry`] or
///   [`cqueue::Entry32`].
pub struct IoUring<S = squeue::Entry, C = cqueue::Entry>
where
    S: squeue::EntryMarker,
    C: cqueue::EntryMarker,
{
    sq: squeue::Inner<S>,
    cq: cqueue::Inner<C>,
    fd: OwnedFd,
    params: Parameters,
    memory: ManuallyDrop<MemoryMap>,
}
