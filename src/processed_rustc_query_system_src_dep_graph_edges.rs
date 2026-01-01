// SRC: ../rust/compiler/rustc_query_system/src/dep_graph/edges.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use std::hash::{Hash, Hasher};
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11 */
use std::ops::Deref;

use smallvec::SmallVec;

use crate::dep_graph::DepNodeIndex;

#[derive(Default, Debug)]
pub(crate) struct EdgesVec {
    max: u32,
    edges: SmallVec<[DepNodeIndex; EdgesVec::INLINE_CAPACITY]>,
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=hash | COMPLEXITY=5 | LINES=7 */

impl Hash for EdgesVec {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        Hash::hash(&self.edges, hasher)
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=5 | LINES=20 */

impl EdgesVec {
    pub(crate) const INLINE_CAPACITY: usize = 8;

    #[inline]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub(crate) fn push(&mut self, edge: DepNodeIndex) {
        self.max = self.max.max(edge.as_u32());
        self.edges.push(edge);
    }

    #[inline]
    pub(crate) fn max_index(&self) -> u32 {
        self.max
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=deref | COMPLEXITY=5 | LINES=9 */

impl Deref for EdgesVec {
    type Target = [DepNodeIndex];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.edges.as_slice()
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=from_iter | COMPLEXITY=8 | LINES=14 */

impl FromIterator<DepNodeIndex> for EdgesVec {
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = DepNodeIndex>,
    {
        let mut vec = EdgesVec::new();
        for index in iter {
            vec.push(index)
        }
        vec
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=extend | COMPLEXITY=8 | LINES=12 */

impl Extend<DepNodeIndex> for EdgesVec {
    #[inline]
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = DepNodeIndex>,
    {
        for elem in iter {
            self.push(elem);
        }
    }
}