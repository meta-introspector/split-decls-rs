macro_rules! deps {
    () => {
        Result!();
        Ok!();
        Error!();
    };
}

macro_rules! __ensure {
    () => {
        deps!();
        macro_rules ! __ensure { ($ ensure : item) => { # [doc = " Return early with an error if a condition is not satisfied."] # [doc = ""] # [doc = " This macro is equivalent to"] # [doc = " <code>if !$cond { return Err([anyhow!($args\\...)][anyhow!]); }</code>."] # [doc = ""] # [doc = " The surrounding function's or closure's return value is required to be"] # [doc = " <code>Result&lt;_, [anyhow::Error][crate::Error]&gt;</code>."] # [doc = ""] # [doc = " Analogously to `assert!`, `ensure!` takes a condition and exits the function"] # [doc = " if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`"] # [doc = " rather than panicking."] # [doc = ""] # [doc = " [anyhow!]: crate::anyhow"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use anyhow::{ensure, Result};"] # [doc = " #"] # [doc = " # fn main() -> Result<()> {"] # [doc = " #     let user = 0;"] # [doc = " #"] # [doc = " ensure!(user == 0, \"only user 0 is allowed\");"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " ```"] # [doc = " # use anyhow::{ensure, Result};"] # [doc = " # use thiserror::Error;"] # [doc = " #"] # [doc = " # const MAX_DEPTH: usize = 1;"] # [doc = " #"] # [doc = " #[derive(Error, Debug)]"] # [doc = " enum ScienceError {"] # [doc = "     #[error(\"recursion limit exceeded\")]"] # [doc = "     RecursionLimitExceeded,"] # [doc = "     # #[error(\"...\")]"] # [doc = "     # More = (stringify! {"] # [doc = "     ..."] # [doc = "     # }, 1).1,"] # [doc = " }"] # [doc = ""] # [doc = " # fn main() -> Result<()> {"] # [doc = " #     let depth = 0;"] # [doc = " #"] # [doc = " ensure!(depth <= MAX_DEPTH, ScienceError::RecursionLimitExceeded);"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " ```"] $ ensure } ; }
    };
}

__ensure!();