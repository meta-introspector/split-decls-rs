macro_rules! deps {
    () => {
        Result!();
        Ok!();
    };
}

macro_rules! bail {
    () => {
        deps!();
        # [doc = " Return early with an error."] # [doc = ""] # [doc = " This macro is equivalent to"] # [doc = " <code>return Err([anyhow!($args\\...)][anyhow!])</code>."] # [doc = ""] # [doc = " The surrounding function's or closure's return value is required to be"] # [doc = " <code>Result&lt;_, [anyhow::Error][crate::Error]&gt;</code>."] # [doc = ""] # [doc = " [anyhow!]: crate::anyhow"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use anyhow::{bail, Result};"] # [doc = " #"] # [doc = " # fn has_permission(user: usize, resource: usize) -> bool {"] # [doc = " #     true"] # [doc = " # }"] # [doc = " #"] # [doc = " # fn main() -> Result<()> {"] # [doc = " #     let user = 0;"] # [doc = " #     let resource = 0;"] # [doc = " #"] # [doc = " if !has_permission(user, resource) {"] # [doc = "     bail!(\"permission denied for accessing {}\", resource);"] # [doc = " }"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " ```"] # [doc = " # use anyhow::{bail, Result};"] # [doc = " # use thiserror::Error;"] # [doc = " #"] # [doc = " # const MAX_DEPTH: usize = 1;"] # [doc = " #"] # [doc = " #[derive(Error, Debug)]"] # [doc = " enum ScienceError {"] # [doc = "     #[error(\"recursion limit exceeded\")]"] # [doc = "     RecursionLimitExceeded,"] # [doc = "     # #[error(\"...\")]"] # [doc = "     # More = (stringify! {"] # [doc = "     ..."] # [doc = "     # }, 1).1,"] # [doc = " }"] # [doc = ""] # [doc = " # fn main() -> Result<()> {"] # [doc = " #     let depth = 0;"] # [doc = " #"] # [doc = " if depth > MAX_DEPTH {"] # [doc = "     bail!(ScienceError::RecursionLimitExceeded);"] # [doc = " }"] # [doc = " #     Ok(())"] # [doc = " # }"] # [doc = " ```"] # [macro_export] # [cfg_attr (not (anyhow_no_clippy_format_args) , clippy :: format_args)] macro_rules ! bail { ($ msg : literal $ (,) ?) => { return $ crate :: __private :: Err ($ crate :: __anyhow ! ($ msg)) } ; ($ err : expr $ (,) ?) => { return $ crate :: __private :: Err ($ crate :: __anyhow ! ($ err)) } ; ($ fmt : expr , $ ($ arg : tt) *) => { return $ crate :: __private :: Err ($ crate :: __anyhow ! ($ fmt , $ ($ arg) *)) } ; }
    };
}

bail!()