macro_rules! deps {
    () => {
        Zsh!();
        Bash!();
        Elvish!();
        Shell!();
        PowerShell!();
        Fish!();
    };
}

macro_rules! shells {
    () => {
        deps!();
        # [doc = " Deprecated, see [`aot`]"] pub mod shells { pub use crate :: aot :: Bash ; pub use crate :: aot :: Elvish ; pub use crate :: aot :: Fish ; pub use crate :: aot :: PowerShell ; pub use crate :: aot :: Shell ; pub use crate :: aot :: Zsh ; }
    };
}

shells!();