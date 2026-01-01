// SRC: ../rust/compiler/rustc_abi/src/extern_abi/tests.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=lookup_Rust | COMPLEXITY=2 | LINES=11 */
use std::assert_matches::assert_matches;
use std::str::FromStr;

use super::*;

#[allow(non_snake_case)]
#[test]
fn lookup_Rust() {
    let abi = ExternAbi::from_str("Rust");
    assert!(abi.is_ok() && abi.unwrap().as_str() == "Rust");
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=lookup_cdecl | COMPLEXITY=2 | LINES=6 */

#[test]
fn lookup_cdecl() {
    let abi = ExternAbi::from_str("cdecl");
    assert!(abi.is_ok() && abi.unwrap().as_str() == "cdecl");
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=lookup_baz | COMPLEXITY=2 | LINES=6 */

#[test]
fn lookup_baz() {
    let abi = ExternAbi::from_str("baz");
    assert_matches!(abi, Err(AbiFromStrErr::Unknown));
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=guarantee_lexicographic_ordering | COMPLEXITY=2 | LINES=8 */

#[test]
fn guarantee_lexicographic_ordering() {
    let abis = ExternAbi::ALL_VARIANTS;
    let mut sorted_abis = abis.to_vec();
    sorted_abis.sort_unstable();
    assert_eq!(abis, sorted_abis);
}