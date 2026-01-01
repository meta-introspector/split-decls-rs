// SRC: ../rust/compiler/rustc_data_structures/src/graph/iterate/tests.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=diamond_post_order | COMPLEXITY=2 | LINES=10 */
use super::super::tests::TestGraph;
use super::*;

#[test]
fn diamond_post_order() {
    let graph = TestGraph::new(0, &[(0, 1), (0, 2), (1, 3), (2, 3)]);

    let result = post_order_from(&graph, 0);
    assert_eq!(result, vec![3, 1, 2, 0]);
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=is_cyclic | COMPLEXITY=2 | LINES=11 */

#[test]
fn is_cyclic() {
    use super::super::is_cyclic;

    let diamond_acyclic = TestGraph::new(0, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
    let diamond_cyclic = TestGraph::new(0, &[(0, 1), (1, 2), (2, 3), (3, 0)]);

    assert!(!is_cyclic(&diamond_acyclic));
    assert!(is_cyclic(&diamond_cyclic));
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=dfs | COMPLEXITY=2 | LINES=8 */

#[test]
fn dfs() {
    let graph = TestGraph::new(0, &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 0)]);

    let result: Vec<usize> = DepthFirstSearch::new(&graph).with_start_node(0).collect();
    assert_eq!(result, vec![0, 2, 3, 1]);
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=dfs_debug | COMPLEXITY=5 | LINES=8 */

#[test]
fn dfs_debug() {
    let graph = TestGraph::new(0, &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 0)]);
    let mut dfs = DepthFirstSearch::new(&graph).with_start_node(0);
    dfs.complete_search();
    assert_eq!(format!("{{0, 1, 2, 3}}"), format!("{:?}", dfs));
}