use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A constant can have reference to other things. Memory map job is holding
/// the necessary bits of memory of the const eval session to keep the constant
/// meaningful.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum MemoryMap<'db> {
    #[default]
    Empty,
    Simple(Box<[u8]>),
    Complex(Box<ComplexMemoryMap<'db>>),
}
