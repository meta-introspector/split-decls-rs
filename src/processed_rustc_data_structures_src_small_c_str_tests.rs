// SRC: ../rust/compiler/rustc_data_structures/src/small_c_str/tests.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=short | COMPLEXITY=2 | LINES=13 */
use super::*;

#[test]
fn short() {
    const TEXT: &str = "abcd";
    let reference = ffi::CString::new(TEXT.to_string()).unwrap();

    let scs = SmallCStr::new(TEXT);

    assert_eq!(scs.len_with_nul(), TEXT.len() + 1);
    assert_eq!(scs.as_c_str(), reference.as_c_str());
    assert!(!scs.spilled());
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=empty | COMPLEXITY=2 | LINES=12 */

#[test]
fn empty() {
    const TEXT: &str = "";
    let reference = ffi::CString::new(TEXT.to_string()).unwrap();

    let scs = SmallCStr::new(TEXT);

    assert_eq!(scs.len_with_nul(), TEXT.len() + 1);
    assert_eq!(scs.as_c_str(), reference.as_c_str());
    assert!(!scs.spilled());
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=long | COMPLEXITY=2 | LINES=14 */

#[test]
fn long() {
    const TEXT: &str = "01234567890123456789012345678901234567890123456789\
                        01234567890123456789012345678901234567890123456789\
                        01234567890123456789012345678901234567890123456789";
    let reference = ffi::CString::new(TEXT.to_string()).unwrap();

    let scs = SmallCStr::new(TEXT);

    assert_eq!(scs.len_with_nul(), TEXT.len() + 1);
    assert_eq!(scs.as_c_str(), reference.as_c_str());
    assert!(scs.spilled());
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=internal_nul | COMPLEXITY=2 | LINES=6 */

#[test]
#[should_panic]
fn internal_nul() {
    let _ = SmallCStr::new("abcd\0def");
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=from_cstr | COMPLEXITY=2 | LINES=8 */

#[test]
fn from_cstr() {
    let c = c"foo";
    let s: SmallCStr = c.into();
    assert_eq!(s.len_with_nul(), 4);
    assert_eq!(s.as_c_str(), c"foo");
}