macro_rules! deps {
    () => {
        WorkspaceDefaultMembers!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl WorkspaceDefaultMembers { # [doc = " Return `true` if the list of workspace default members is supported by"] # [doc = " the called cargo-metadata version and `false` otherwise."] # [doc = ""] # [doc = " In particular useful when parsing the output of `cargo-metadata` for"] # [doc = " versions of Cargo < 1.71, as dereferencing [`WorkspaceDefaultMembers`]"] # [doc = " for these versions will panic."] # [doc = ""] # [doc = " Opposite of [`WorkspaceDefaultMembers::is_missing`]."] pub fn is_available (& self) -> bool { self . 0 . is_some () } # [doc = " Return `false` if the list of workspace default members is supported by"] # [doc = " the called cargo-metadata version and `true` otherwise."] # [doc = ""] # [doc = " In particular useful when parsing the output of `cargo-metadata` for"] # [doc = " versions of Cargo < 1.71, as dereferencing [`WorkspaceDefaultMembers`]"] # [doc = " for these versions will panic."] # [doc = ""] # [doc = " Opposite of [`WorkspaceDefaultMembers::is_available`]."] pub fn is_missing (& self) -> bool { self . 0 . is_none () } }
    };
}

impl_15!()