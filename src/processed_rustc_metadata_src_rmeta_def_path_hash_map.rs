// SRC: ../rust/compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_data_structures::owned_slice::OwnedSlice;
use crate::rustc_complete::def_path_hash_map::{Config as HashMapConfig, DefPathHashMap};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::rustc_complete::def_id::{DefIndex, DefPathHash};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

use crate::rmeta::{DecodeContext, EncodeContext};
/* AST_META: AST_ID=5 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */

pub(crate) enum DefPathHashMapRef<'tcx> {
    OwnedFromMetadata(odht::HashTable<HashMapConfig, OwnedSlice>),
    BorrowedFromTcx(&'tcx DefPathHashMap),
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=11 | LINES=14 */

impl DefPathHashMapRef<'_> {
    #[inline]
    pub(crate) fn def_path_hash_to_def_index(&self, def_path_hash: &DefPathHash) -> DefIndex {
        match *self {
            DefPathHashMapRef::OwnedFromMetadata(ref map) => {
                map.get(&def_path_hash.local_hash()).unwrap()
            }
            DefPathHashMapRef::BorrowedFromTcx(_) => {
                panic!("DefPathHashMap::BorrowedFromTcx variant only exists for serialization")
            }
        }
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=encode | COMPLEXITY=14 | LINES=15 */

impl<'a, 'tcx> Encodable<EncodeContext<'a, 'tcx>> for DefPathHashMapRef<'tcx> {
    fn encode(&self, e: &mut EncodeContext<'a, 'tcx>) {
        match *self {
            DefPathHashMapRef::BorrowedFromTcx(def_path_hash_map) => {
                let bytes = def_path_hash_map.raw_bytes();
                e.emit_usize(bytes.len());
                e.emit_raw_bytes(bytes);
            }
            DefPathHashMapRef::OwnedFromMetadata(_) => {
                panic!("DefPathHashMap::OwnedFromMetadata variant only exists for deserialization")
            }
        }
    }
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=decode | COMPLEXITY=10 | LINES=18 */

impl<'a, 'tcx> Decodable<DecodeContext<'a, 'tcx>> for DefPathHashMapRef<'static> {
    fn decode(d: &mut DecodeContext<'a, 'tcx>) -> DefPathHashMapRef<'static> {
        let len = d.read_usize();
        let pos = d.position();
        let o = d.blob().bytes().clone().slice(|blob| &blob[pos..pos + len]);

        // Although we already have the data we need via the `OwnedSlice`, we still need
        // to advance the `DecodeContext`'s position so it's in a valid state after
        // the method. We use `read_raw_bytes()` for that.
        let _ = d.read_raw_bytes(len);

        let inner = odht::HashTable::from_raw_bytes(o).unwrap_or_else(|e| {
            panic!("decode error: {e}");
        });
        DefPathHashMapRef::OwnedFromMetadata(inner)
    }
}