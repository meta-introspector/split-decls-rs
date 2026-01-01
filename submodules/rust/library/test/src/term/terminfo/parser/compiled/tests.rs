// SRC: ../rust/library/test/src/term/terminfo/parser/compiled/tests.rs
use super::*;

#[test]
fn test_veclens() {
    assert_eq!(boolfnames.len(), boolnames.len());
    assert_eq!(numfnames.len(), numnames.len());
    assert_eq!(stringfnames.len(), stringnames.len());
}
