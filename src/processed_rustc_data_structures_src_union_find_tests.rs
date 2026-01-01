// SRC: ../rust/compiler/rustc_data_structures/src/union_find/tests.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=empty | COMPLEXITY=5 | LINES=10 */
use super::UnionFind;

#[test]
fn empty() {
    let mut sets = UnionFind::<u32>::new(10);

    for i in 1..10 {
        assert_eq!(sets.find(i), i);
    }
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=transitive | COMPLEXITY=5 | LINES=22 */

#[test]
fn transitive() {
    let mut sets = UnionFind::<u32>::new(10);

    sets.unify(3, 7);
    sets.unify(4, 2);

    assert_eq!(sets.find(7), sets.find(3));
    assert_eq!(sets.find(2), sets.find(4));
    assert_ne!(sets.find(3), sets.find(4));

    sets.unify(7, 4);

    assert_eq!(sets.find(7), sets.find(3));
    assert_eq!(sets.find(2), sets.find(4));
    assert_eq!(sets.find(3), sets.find(4));

    for i in [0, 1, 5, 6, 8, 9] {
        assert_eq!(sets.find(i), i);
    }
}