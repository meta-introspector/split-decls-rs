// SRC: ../rust/compiler/rustc_public/src/error.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */
// When things go wrong, we need some error handling.
// There are a few different types of errors in rustc_public:
//
// - [CompilerError]: This represents errors that can be raised when invoking the compiler.
// - [Error]: Generic error that represents the reason why a request that could not be fulfilled.

use std::fmt::{Debug, Display, Formatter};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use std::{fmt, io};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=9 | LINES=7 */

use crate::rustc_public_bridge::bridge;

macro_rules! error {
     ($fmt: literal $(,)?) => { Error(format!($fmt)) };
     ($fmt: literal, $($arg:tt)*) => { Error(format!($fmt, $($arg)*)) };
}
/* AST_META: AST_ID=4 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=14 */

pub(crate) use error;

/// An error type used to represent an error that has already been reported by the compiler.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CompilerError<T> {
    /// Compilation failed, either due to normal errors or ICE.
    Failed,
    /// Compilation was interrupted.
    Interrupted(T),
    /// Compilation skipped. This happens when users invoke rustc to retrieve information such as
    /// --version.
    Skipped,
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=Error(pub(crate) | COMPLEXITY=7 | LINES=14 */

/// A generic error to represent an API request that cannot be fulfilled.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error(pub(crate) String);

impl bridge::Error for Error {
    fn new(msg: String) -> Self {
        Self(msg)
    }

    fn from_internal<T: Debug>(err: T) -> Self {
        Self(format!("{err:?}"))
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=6 */

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6 */

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=10 | LINES=13 */

impl<T> Display for CompilerError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::Failed => write!(f, "Compilation Failed"),
            CompilerError::Interrupted(reason) => write!(f, "Compilation Interrupted: {reason}"),
            CompilerError::Skipped => write!(f, "Compilation Skipped"),
        }
    }
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=10 | LINES=13 */

impl<T> Debug for CompilerError<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::Failed => write!(f, "Compilation Failed"),
            CompilerError::Interrupted(reason) => write!(f, "Compilation Interrupted: {reason:?}"),
            CompilerError::Skipped => write!(f, "Compilation Skipped"),
        }
    }
}
/* AST_META: AST_ID=10 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl std::error::Error for Error {}
/* AST_META: AST_ID=11 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl<T> std::error::Error for CompilerError<T> where T: Display + Debug {}
/* AST_META: AST_ID=12 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=6 */

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error(value.to_string())
    }
}