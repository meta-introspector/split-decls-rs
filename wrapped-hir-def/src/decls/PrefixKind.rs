macro_rules! PrefixKind {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum PrefixKind { # [doc = " Causes paths to always start with either `self`, `super`, `crate` or a crate-name."] # [doc = " This is the same as plain, just that paths will start with `self` prepended if the path"] # [doc = " starts with an identifier that is not a crate."] BySelf , # [doc = " Causes paths to not use a self, super or crate prefix."] Plain , # [doc = " Causes paths to start with `crate` where applicable, effectively forcing paths to be absolute."] ByCrate , }
    };
}

PrefixKind!();