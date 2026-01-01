// SRC: ../rust/compiler/rustc_codegen_ssa/src/back/linker/tests.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=test_rpaths_to_args | COMPLEXITY=2 | LINES=8 */
use super::*;

#[test]
fn test_rpaths_to_args() {
    let mut cmd = Command::new("foo");
    convert_link_args_to_cc_args(&mut cmd, &["-rpath", "path1", "-rpath", "path2"]);
    assert_eq!(cmd.get_args(), [OsStr::new("-Wl,-rpath,path1,-rpath,path2")]);
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=test_xlinker | COMPLEXITY=3 | LINES=23 */

#[test]
fn test_xlinker() {
    let mut cmd = Command::new("foo");
    convert_link_args_to_cc_args(
        &mut cmd,
        &["arg1", "arg2", "arg3,with,comma", "arg4,with,comma", "arg5", "arg6,with,comma"],
    );

    assert_eq!(
        cmd.get_args(),
        [
            OsStr::new("-Wl,arg1,arg2"),
            OsStr::new("-Xlinker"),
            OsStr::new("arg3,with,comma"),
            OsStr::new("-Xlinker"),
            OsStr::new("arg4,with,comma"),
            OsStr::new("-Wl,arg5"),
            OsStr::new("-Xlinker"),
            OsStr::new("arg6,with,comma"),
        ]
    );
}