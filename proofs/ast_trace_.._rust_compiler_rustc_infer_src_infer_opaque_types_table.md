# AST Trace: ../rust/compiler/rustc_infer/src/infer/opaque_types/table.rs

Generated 10 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
use std::ops::Deref;

use rustc_data_structures::fx::FxIndexMap;
use rustc_data_structures::undo_log::UndoLogs;
use rustc_middle::bug;
use rustc_middle::ty::{self, OpaqueHiddenType, OpaqueTypeKey, Ty};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use tracing::instrument;

use crate::infer::snapshot::undo_log::{InferCtxtUndoLogs, UndoLog};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=STRUCT | NAME=OpaqueTypeStorage | COMPLEXITY=2 | LINES=6

```rust
#[derive(Default, Debug, Clone)]
pub struct OpaqueTypeStorage<'tcx> {
    opaque_types: FxIndexMap<OpaqueTypeKey<'tcx>, OpaqueHiddenType<'tcx>>,
    duplicate_entries: Vec<(OpaqueTypeKey<'tcx>, OpaqueHiddenType<'tcx>)>,
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=STRUCT | NAME=OpaqueTypeStorageEntries | COMPLEXITY=2 | LINES=10

```rust
/// The number of entries in the opaque type storage at a given point.
///
/// Used to check that we haven't added any new opaque types after checking
/// the opaque types currently in the storage.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpaqueTypeStorageEntries {
    opaque_types: usize,
    duplicate_entries: usize,
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=needs_reevaluation | COMPLEXITY=5 | LINES=6

```rust
impl rustc_type_ir::inherent::OpaqueTypeStorageEntries for OpaqueTypeStorageEntries {
    fn needs_reevaluation(self, canonicalized: usize) -> bool {
        self.opaque_types != canonicalized
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=is_empty | COMPLEXITY=35 | LINES=91

```rust
impl<'tcx> OpaqueTypeStorage<'tcx> {
    #[instrument(level = "debug")]
    pub(crate) fn remove(
        &mut self,
        key: OpaqueTypeKey<'tcx>,
        prev: Option<OpaqueHiddenType<'tcx>>,
    ) {
        if let Some(prev) = prev {
            *self.opaque_types.get_mut(&key).unwrap() = prev;
        } else {
            // FIXME(#120456) - is `swap_remove` correct?
            match self.opaque_types.swap_remove(&key) {
                None => bug!("reverted opaque type inference that was never registered: {:?}", key),
                Some(_) => {}
            }
        }
    }

    pub(crate) fn pop_duplicate_entry(&mut self) {
        let entry = self.duplicate_entries.pop();
        assert!(entry.is_some());
    }

    pub fn is_empty(&self) -> bool {
        let OpaqueTypeStorage { opaque_types, duplicate_entries } = self;
        opaque_types.is_empty() && duplicate_entries.is_empty()
    }

    pub(crate) fn take_opaque_types(
        &mut self,
    ) -> impl Iterator<Item = (OpaqueTypeKey<'tcx>, OpaqueHiddenType<'tcx>)> {
        let OpaqueTypeStorage { opaque_types, duplicate_entries } = self;
        std::mem::take(opaque_types).into_iter().chain(std::mem::take(duplicate_entries))
    }

    pub fn num_entries(&self) -> OpaqueTypeStorageEntries {
        OpaqueTypeStorageEntries {
            opaque_types: self.opaque_types.len(),
            duplicate_entries: self.duplicate_entries.len(),
        }
    }

    pub fn opaque_types_added_since(
        &self,
        prev_entries: OpaqueTypeStorageEntries,
    ) -> impl Iterator<Item = (OpaqueTypeKey<'tcx>, OpaqueHiddenType<'tcx>)> {
        self.opaque_types
            .iter()
            .skip(prev_entries.opaque_types)
            .map(|(k, v)| (*k, *v))
            .chain(self.duplicate_entries.iter().skip(prev_entries.duplicate_entries).copied())
    }

    /// Only returns the opaque types from the lookup table. These are used
    /// when normalizing opaque types and have a unique key.
    ///
    /// Outside of canonicalization one should generally use `iter_opaque_types`
    /// to also consider duplicate entries.
    pub fn iter_lookup_table(
        &self,
    ) -> impl Iterator<Item = (OpaqueTypeKey<'tcx>, OpaqueHiddenType<'tcx>)> {
        self.opaque_types.iter().map(|(k, v)| (*k, *v))
    }

    /// Only returns the opaque types which are stored in `duplicate_entries`.
    ///
    /// These have to considered when checking all opaque type uses but are e.g.
    /// irrelevant for canonical inputs as nested queries never meaningfully
    /// accesses them.
    pub fn iter_duplicate_entries(
        &self,
    ) -> impl Iterator<Item = (OpaqueTypeKey<'tcx>, OpaqueHiddenType<'tcx>)> {
        self.duplicate_entries.iter().copied()
    }

    pub fn iter_opaque_types(
        &self,
    ) -> impl Iterator<Item = (OpaqueTypeKey<'tcx>, OpaqueHiddenType<'tcx>)> {
        let OpaqueTypeStorage { opaque_types, duplicate_entries } = self;
        opaque_types.iter().map(|(k, v)| (*k, *v)).chain(duplicate_entries.iter().copied())
    }

    #[inline]
    pub(crate) fn with_log<'a>(
        &'a mut self,
        undo_log: &'a mut InferCtxtUndoLogs<'tcx>,
    ) -> OpaqueTypeTable<'a, 'tcx> {
        OpaqueTypeTable { storage: self, undo_log }
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=drop | COMPLEXITY=9 | LINES=8

```rust
impl<'tcx> Drop for OpaqueTypeStorage<'tcx> {
    fn drop(&mut self) {
        if !self.is_empty() {
            ty::tls::with(|tcx| tcx.dcx().delayed_bug(format!("{:?}", self.opaque_types)));
        }
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=STRUCT | NAME=OpaqueTypeTable | COMPLEXITY=2 | LINES=6

```rust
pub struct OpaqueTypeTable<'a, 'tcx> {
    storage: &'a mut OpaqueTypeStorage<'tcx>,

    undo_log: &'a mut InferCtxtUndoLogs<'tcx>,
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=deref | COMPLEXITY=5 | LINES=6

```rust
impl<'tcx> Deref for OpaqueTypeTable<'_, 'tcx> {
    type Target = OpaqueTypeStorage<'tcx>;
    fn deref(&self) -> &Self::Target {
        self.storage
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=register | COMPLEXITY=8 | LINES=23

```rust
impl<'a, 'tcx> OpaqueTypeTable<'a, 'tcx> {
    #[instrument(skip(self), level = "debug")]
    pub fn register(
        &mut self,
        key: OpaqueTypeKey<'tcx>,
        hidden_type: OpaqueHiddenType<'tcx>,
    ) -> Option<Ty<'tcx>> {
        if let Some(entry) = self.storage.opaque_types.get_mut(&key) {
            let prev = std::mem::replace(entry, hidden_type);
            self.undo_log.push(UndoLog::OpaqueTypes(key, Some(prev)));
            return Some(prev.ty);
        }
        self.storage.opaque_types.insert(key, hidden_type);
        self.undo_log.push(UndoLog::OpaqueTypes(key, None));
        None
    }

    pub fn add_duplicate(&mut self, key: OpaqueTypeKey<'tcx>, hidden_type: OpaqueHiddenType<'tcx>) {
        self.storage.duplicate_entries.push((key, hidden_type));
        self.undo_log.push(UndoLog::DuplicateOpaqueType);
    }
}
```

---
*Generated by AST tracing system*
