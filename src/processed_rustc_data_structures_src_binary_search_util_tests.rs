// SRC: ../rust/compiler/rustc_data_structures/src/binary_search_util/tests.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=test_map | COMPLEXITY=2 | LINES=9 */
use super::*;

type Element = (usize, &'static str);

fn test_map() -> Vec<Element> {
    let mut data = vec![(3, "three-a"), (0, "zero"), (3, "three-b"), (22, "twenty-two")];
    data.sort_by_key(get_key);
    data
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=get_key | COMPLEXITY=2 | LINES=4 */

fn get_key(data: &Element) -> usize {
    data.0
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=binary_search_slice_test | COMPLEXITY=2 | LINES=10 */

#[test]
fn binary_search_slice_test() {
    let map = test_map();
    assert_eq!(binary_search_slice(&map, get_key, &0), &[(0, "zero")]);
    assert_eq!(binary_search_slice(&map, get_key, &1), &[]);
    assert_eq!(binary_search_slice(&map, get_key, &3), &[(3, "three-a"), (3, "three-b")]);
    assert_eq!(binary_search_slice(&map, get_key, &22), &[(22, "twenty-two")]);
    assert_eq!(binary_search_slice(&map, get_key, &23), &[]);
}