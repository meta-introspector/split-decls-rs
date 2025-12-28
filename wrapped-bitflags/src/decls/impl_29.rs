macro_rules! deps {
    () => {
        Flag!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < B > Flag < B > { # [doc = "\n    Define a flag.\n\n    If `name` is non-empty then the flag is named, otherwise it's unnamed.\n    "] pub const fn new (name : & 'static str , value : B) -> Self { Flag { name , value } } # [doc = "\n    Get the name of this flag.\n\n    If the flag is unnamed then the returned string will be empty.\n    "] pub const fn name (& self) -> & 'static str { self . name } # [doc = "\n    Get the flags value of this flag.\n    "] pub const fn value (& self) -> & B { & self . value } # [doc = "\n    Whether the flag is named.\n\n    If [`Flag::name`] returns a non-empty string then this method will return `true`.\n    "] pub const fn is_named (& self) -> bool { ! self . name . is_empty () } # [doc = "\n    Whether the flag is unnamed.\n\n    If [`Flag::name`] returns a non-empty string then this method will return `false`.\n    "] pub const fn is_unnamed (& self) -> bool { self . name . is_empty () } }
    };
}

impl_29!()