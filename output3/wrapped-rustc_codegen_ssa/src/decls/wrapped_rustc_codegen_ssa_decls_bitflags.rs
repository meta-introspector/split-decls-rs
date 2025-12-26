use serde::{Deserialize, Serialize};
use std::collections::HashMap;
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct MemFlags : u8 { const
    VOLATILE = 1 << 0; const NONTEMPORAL = 1 << 1; const UNALIGNED = 1 << 2; }
}
