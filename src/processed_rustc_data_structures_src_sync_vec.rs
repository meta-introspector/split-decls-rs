// SRC: ../rust/compiler/rustc_data_structures/src/sync/vec.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=AppendOnlyIndexVec | COMPLEXITY=2 | LINES=9 */
use std::marker::PhantomData;

use crate::rustc_index::Idx;

#[derive(Default)]
pub struct AppendOnlyIndexVec<I: Idx, T: Copy> {
    vec: elsa::sync::LockFreeFrozenVec<T>,
    _marker: PhantomData<fn(&I)>,
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=new | COMPLEXITY=6 | LINES=16 */

impl<I: Idx, T: Copy> AppendOnlyIndexVec<I, T> {
    pub fn new() -> Self {
        Self { vec: elsa::sync::LockFreeFrozenVec::new(), _marker: PhantomData }
    }

    pub fn push(&self, val: T) -> I {
        let i = self.vec.push(val);
        I::new(i)
    }

    pub fn get(&self, i: I) -> Option<T> {
        let i = i.index();
        self.vec.get(i)
    }
}
/* AST_META: AST_ID=3 | TYPE=STRUCT | NAME=AppendOnlyVec | COMPLEXITY=2 | LINES=5 */

#[derive(Default)]
pub struct AppendOnlyVec<T: Copy> {
    vec: parking_lot::RwLock<Vec<T>>,
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=new | COMPLEXITY=9 | LINES=28 */

impl<T: Copy> AppendOnlyVec<T> {
    pub fn new() -> Self {
        Self { vec: Default::default() }
    }

    pub fn push(&self, val: T) -> usize {
        let mut v = self.vec.write();
        let n = v.len();
        v.push(val);
        n
    }

    pub fn get(&self, i: usize) -> Option<T> {
        self.vec.read().get(i).copied()
    }

    pub fn iter_enumerated(&self) -> impl Iterator<Item = (usize, T)> {
        (0..)
            .map(|i| (i, self.get(i)))
            .take_while(|(_, o)| o.is_some())
            .filter_map(|(i, o)| Some((i, o?)))
    }

    pub fn iter(&self) -> impl Iterator<Item = T> {
        (0..).map(|i| self.get(i)).take_while(|o| o.is_some()).flatten()
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=contains | COMPLEXITY=3 | LINES=6 */

impl<T: Copy + PartialEq> AppendOnlyVec<T> {
    pub fn contains(&self, val: T) -> bool {
        self.iter_enumerated().any(|(_, v)| v == val)
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=from_iter | COMPLEXITY=8 | LINES=10 */

impl<A: Copy> FromIterator<A> for AppendOnlyVec<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let this = Self::new();
        for val in iter {
            this.push(val);
        }
        this
    }
}