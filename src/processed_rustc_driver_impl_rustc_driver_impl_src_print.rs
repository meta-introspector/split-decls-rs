// SRC: ../rust/compiler/rustc_driver_impl/src/print.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use std::fmt;
use std::io::{self, Write as _};
/* AST_META: AST_ID=2 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=9 | LINES=6 */

macro_rules! safe_print {
    ($($arg:tt)*) => {{
        $crate::print::print(std::format_args!($($arg)*));
    }};
}
/* AST_META: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=9 | LINES=6 */

macro_rules! safe_println {
    ($($arg:tt)*) => {
        safe_print!("{}\n", std::format_args!($($arg)*))
    };
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=5 | LINES=6 */

pub(crate) fn print(args: fmt::Arguments<'_>) {
    if let Err(_) = io::stdout().write_fmt(args) {
        crate::rustc_errors::FatalError.raise();
    }
}