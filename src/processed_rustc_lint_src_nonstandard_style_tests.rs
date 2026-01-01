// SRC: ../rust/compiler/rustc_lint/src/nonstandard_style/tests.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use super::{is_camel_case, to_camel_case};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=camel_case | COMPLEXITY=3 | LINES=20 */

#[test]
fn camel_case() {
    assert!(!is_camel_case("userData"));
    assert_eq!(to_camel_case("userData"), "UserData");

    assert!(is_camel_case("X86_64"));

    assert!(!is_camel_case("X86__64"));
    assert_eq!(to_camel_case("X86__64"), "X86_64");

    assert!(!is_camel_case("Abc_123"));
    assert_eq!(to_camel_case("Abc_123"), "Abc123");

    assert!(!is_camel_case("A1_b2_c3"));
    assert_eq!(to_camel_case("A1_b2_c3"), "A1B2C3");

    assert!(!is_camel_case("ONE_TWO_THREE"));
    assert_eq!(to_camel_case("ONE_TWO_THREE"), "OneTwoThree");
}