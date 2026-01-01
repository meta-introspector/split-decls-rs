/* FP:mod.rs-0001 */ use std::borrow::{Borrow, BorrowMut};
/* FP:mod.rs-0002 */ use std::hash::Hash;
/* FP:mod.rs-0003 */ use std::marker::PhantomData;
/* FP:mod.rs-0004 */ use std::ops;
/* FP:mod.rs-0005 */ 
/* FP:mod.rs-0006 */ use crate::fx::FxHashMap;
/* FP:mod.rs-0007 */ pub use crate::undo_log::Snapshot;
/* FP:mod.rs-0008 */ use crate::undo_log::{Rollback, Snapshots, UndoLogs, VecLog};
/* FP:mod.rs-0009 */ 
/* FP:mod.rs-0010 */ #[cfg(test)]
/* FP:mod.rs-0012 */ 
/* FP:mod.rs-0013 */ pub type SnapshotMapStorage<K, V> = SnapshotMap<K, V, FxHashMap<K, V>, ()>;
/* FP:mod.rs-0014 */ pub type SnapshotMapRef<'a, K, V, L> = SnapshotMap<K, V, &'a mut FxHashMap<K, V>, &'a mut L>;
/* FP:mod.rs-0015 */ 
/* FP:mod.rs-0016 */ #[derive(Clone)]
/* FP:mod.rs-0017 */ pub struct SnapshotMap<K, V, M = FxHashMap<K, V>, L = VecLog<UndoLog<K, V>>> {
/* FP:mod.rs-0018 */     map: M,
/* FP:mod.rs-0019 */     undo_log: L,
/* FP:mod.rs-0020 */     _marker: PhantomData<(K, V)>,
/* FP:mod.rs-0021 */ }
/* FP:mod.rs-0022 */ 
/* FP:mod.rs-0023 */ // HACK(eddyb) manual impl avoids `Default` bounds on `K` and `V`.
/* FP:mod.rs-0024 */ impl<K, V, M, L> Default for SnapshotMap<K, V, M, L>
/* FP:mod.rs-0025 */ where
/* FP:mod.rs-0026 */     M: Default,
/* FP:mod.rs-0027 */     L: Default,
/* FP:mod.rs-0028 */ {
/* FP:mod.rs-0029 */     fn default() -> Self {
/* FP:mod.rs-0030 */         SnapshotMap { map: Default::default(), undo_log: Default::default(), _marker: PhantomData }
/* FP:mod.rs-0031 */     }
/* FP:mod.rs-0032 */ }
/* FP:mod.rs-0033 */ 
/* FP:mod.rs-0034 */ #[derive(Clone)]
/* FP:mod.rs-0035 */ pub enum UndoLog<K, V> {
/* FP:mod.rs-0036 */     Inserted(K),
/* FP:mod.rs-0037 */     Overwrite(K, V),
/* FP:mod.rs-0038 */     Purged,
/* FP:mod.rs-0039 */ }
/* FP:mod.rs-0040 */ 
/* FP:mod.rs-0041 */ impl<K, V, M, L> SnapshotMap<K, V, M, L> {
/* FP:mod.rs-0042 */     #[inline]
/* FP:mod.rs-0043 */     pub fn with_log<L2>(&mut self, undo_log: L2) -> SnapshotMap<K, V, &mut M, L2> {
/* FP:mod.rs-0044 */         SnapshotMap { map: &mut self.map, undo_log, _marker: PhantomData }
/* FP:mod.rs-0045 */     }
/* FP:mod.rs-0046 */ }
/* FP:mod.rs-0047 */ 
/* FP:mod.rs-0048 */ impl<K, V, M, L> SnapshotMap<K, V, M, L>
/* FP:mod.rs-0049 */ where
/* FP:mod.rs-0050 */     K: Hash + Clone + Eq,
/* FP:mod.rs-0051 */     M: BorrowMut<FxHashMap<K, V>> + Borrow<FxHashMap<K, V>>,
/* FP:mod.rs-0052 */     L: UndoLogs<UndoLog<K, V>>,
/* FP:mod.rs-0053 */ {
/* FP:mod.rs-0054 */     pub fn clear(&mut self) {
/* FP:mod.rs-0055 */         self.map.borrow_mut().clear();
/* FP:mod.rs-0056 */         self.undo_log.clear();
/* FP:mod.rs-0057 */     }
/* FP:mod.rs-0058 */ 
/* FP:mod.rs-0059 */     pub fn insert(&mut self, key: K, value: V) -> bool {
/* FP:mod.rs-0060 */         match self.map.borrow_mut().insert(key.clone(), value) {
/* FP:mod.rs-0061 */             None => {
/* FP:mod.rs-0062 */                 self.undo_log.push(UndoLog::Inserted(key));
/* FP:mod.rs-0063 */                 true
/* FP:mod.rs-0064 */             }
/* FP:mod.rs-0065 */             Some(old_value) => {
/* FP:mod.rs-0066 */                 self.undo_log.push(UndoLog::Overwrite(key, old_value));
/* FP:mod.rs-0067 */                 false
/* FP:mod.rs-0068 */             }
/* FP:mod.rs-0069 */         }
/* FP:mod.rs-0070 */     }
/* FP:mod.rs-0071 */ 
/* FP:mod.rs-0072 */     pub fn remove(&mut self, key: K) -> bool {
/* FP:mod.rs-0073 */         match self.map.borrow_mut().remove(&key) {
/* FP:mod.rs-0074 */             Some(old_value) => {
/* FP:mod.rs-0075 */                 self.undo_log.push(UndoLog::Overwrite(key, old_value));
/* FP:mod.rs-0076 */                 true
/* FP:mod.rs-0077 */             }
/* FP:mod.rs-0078 */             None => false,
/* FP:mod.rs-0079 */         }
/* FP:mod.rs-0080 */     }
/* FP:mod.rs-0081 */ 
/* FP:mod.rs-0082 */     pub fn get(&self, key: &K) -> Option<&V> {
/* FP:mod.rs-0083 */         self.map.borrow().get(key)
/* FP:mod.rs-0084 */     }
/* FP:mod.rs-0085 */ }
/* FP:mod.rs-0086 */ 
/* FP:mod.rs-0087 */ impl<K, V> SnapshotMap<K, V>
/* FP:mod.rs-0088 */ where
/* FP:mod.rs-0089 */     K: Hash + Clone + Eq,
/* FP:mod.rs-0090 */ {
/* FP:mod.rs-0091 */     pub fn snapshot(&mut self) -> Snapshot {
/* FP:mod.rs-0092 */         self.undo_log.start_snapshot()
/* FP:mod.rs-0093 */     }
/* FP:mod.rs-0094 */ 
/* FP:mod.rs-0095 */     pub fn commit(&mut self, snapshot: Snapshot) {
/* FP:mod.rs-0096 */         self.undo_log.commit(snapshot)
/* FP:mod.rs-0097 */     }
/* FP:mod.rs-0098 */ 
/* FP:mod.rs-0099 */     pub fn rollback_to(&mut self, snapshot: Snapshot) {
/* FP:mod.rs-0100 */         let map = &mut self.map;
/* FP:mod.rs-0101 */         self.undo_log.rollback_to(|| map, snapshot)
/* FP:mod.rs-0102 */     }
/* FP:mod.rs-0103 */ }
/* FP:mod.rs-0104 */ 
/* FP:mod.rs-0105 */ impl<'k, K, V, M, L> ops::Index<&'k K> for SnapshotMap<K, V, M, L>
/* FP:mod.rs-0106 */ where
/* FP:mod.rs-0107 */     K: Hash + Clone + Eq,
/* FP:mod.rs-0108 */     M: Borrow<FxHashMap<K, V>>,
/* FP:mod.rs-0109 */ {
/* FP:mod.rs-0110 */     type Output = V;
/* FP:mod.rs-0111 */     fn index(&self, key: &'k K) -> &V {
/* FP:mod.rs-0112 */         &self.map.borrow()[key]
/* FP:mod.rs-0113 */     }
/* FP:mod.rs-0114 */ }
/* FP:mod.rs-0115 */ 
/* FP:mod.rs-0116 */ impl<K, V, M, L> Rollback<UndoLog<K, V>> for SnapshotMap<K, V, M, L>
/* FP:mod.rs-0117 */ where
/* FP:mod.rs-0118 */     K: Eq + Hash,
/* FP:mod.rs-0119 */     M: Rollback<UndoLog<K, V>>,
/* FP:mod.rs-0120 */ {
/* FP:mod.rs-0121 */     fn reverse(&mut self, undo: UndoLog<K, V>) {
/* FP:mod.rs-0122 */         self.map.reverse(undo)
/* FP:mod.rs-0123 */     }
/* FP:mod.rs-0124 */ }
/* FP:mod.rs-0125 */ 
/* FP:mod.rs-0126 */ impl<K, V> Rollback<UndoLog<K, V>> for FxHashMap<K, V>
/* FP:mod.rs-0127 */ where
/* FP:mod.rs-0128 */     K: Eq + Hash,
/* FP:mod.rs-0129 */ {
/* FP:mod.rs-0130 */     fn reverse(&mut self, undo: UndoLog<K, V>) {
/* FP:mod.rs-0131 */         match undo {
/* FP:mod.rs-0132 */             UndoLog::Inserted(key) => {
/* FP:mod.rs-0133 */                 self.remove(&key);
/* FP:mod.rs-0134 */             }
/* FP:mod.rs-0135 */ 
/* FP:mod.rs-0136 */             UndoLog::Overwrite(key, old_value) => {
/* FP:mod.rs-0137 */                 self.insert(key, old_value);
/* FP:mod.rs-0138 */             }
/* FP:mod.rs-0139 */ 
/* FP:mod.rs-0140 */             UndoLog::Purged => {}
/* FP:mod.rs-0141 */         }
/* FP:mod.rs-0142 */     }
/* FP:mod.rs-0143 */ }