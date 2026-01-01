// SRC: ../rust/compiler/rustc_data_structures/src/flock.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=10 | LINES=28 */
// Simple file-locking apis for each OS.
//
// This is not meant to be in the standard library, it does nothing with
// green/native threading. This is just a bare-bones enough solution for
// librustdoc, it is not production quality at all.

cfg_select! {
    target_os = "linux" => {
        use linux as imp;
    }
    target_os = "redox" => {
        use linux as imp;
    }
    unix => {
        use unix as imp;
    }
    windows => {
        use self::windows as imp;
    }
    _ => {
        use unsupported as imp;
    }
}
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=1 | LINES=2 */

pub use imp::Lock;