use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'db> MemoryMap<'db> {
    pub fn vtable_ty(&self, id: usize) -> Result<Ty<'db>, MirEvalError<'db>> {
        match self {
            MemoryMap::Empty | MemoryMap::Simple(_) => Err(MirEvalError::InvalidVTableId(id)),
            MemoryMap::Complex(cm) => cm.vtable.ty(id),
        }
    }
    fn simple(v: Box<[u8]>) -> Self {
        MemoryMap::Simple(v)
    }
    /// This functions convert each address by a function `f` which gets the byte intervals and assign an address
    /// to them. It is useful when you want to load a constant with a memory map in a new memory. You can pass an
    /// allocator function as `f` and it will return a mapping of old addresses to new addresses.
    fn transform_addresses(
        &self,
        mut f: impl FnMut(&[u8], usize) -> Result<usize, MirEvalError<'db>>,
    ) -> Result<FxHashMap<usize, usize>, MirEvalError<'db>> {
        let mut transform = |(addr, val): (&usize, &[u8])| {
            let addr = *addr;
            let align = if addr == 0 {
                64
            } else {
                (addr - (addr & (addr - 1))).min(64)
            };
            f(val, align).map(|it| (addr, it))
        };
        match self {
            MemoryMap::Empty => Ok(Default::default()),
            MemoryMap::Simple(m) => transform((&0, m)).map(|(addr, val)| {
                let mut map = FxHashMap::with_capacity_and_hasher(1, rustc_hash::FxBuildHasher);
                map.insert(addr, val);
                map
            }),
            MemoryMap::Complex(cm) => cm
                .memory
                .iter()
                .map(|(addr, val)| transform((addr, val)))
                .collect(),
        }
    }
    fn get(&self, addr: usize, size: usize) -> Option<&[u8]> {
        if size == 0 {
            Some(&[])
        } else {
            match self {
                MemoryMap::Empty => Some(&[]),
                MemoryMap::Simple(m) if addr == 0 => m.get(0..size),
                MemoryMap::Simple(_) => None,
                MemoryMap::Complex(cm) => cm.memory.get(&addr)?.get(0..size),
            }
        }
    }
}
