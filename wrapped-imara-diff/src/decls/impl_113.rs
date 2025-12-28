macro_rules! deps {
    () => {
        Hunk!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl Hunk { # [doc = " Can be used instead of `Option::None` for better performance."] # [doc = " Because `imara-diff` does not support more then `i32::MAX` there is an unused bit pattern that can be used."] # [doc = " Has some nice properties where it usually is not necessary to check for `None` separately:"] # [doc = " Empty ranges fail contains checks and also fail smaller then checks."] pub const NONE : Hunk = Hunk { before : u32 :: MAX .. u32 :: MAX , after : u32 :: MAX .. u32 :: MAX , } ; # [doc = " Inverts a hunk so that it represents a change"] # [doc = " that would undo this hunk."] pub fn invert (& self) -> Hunk { Hunk { before : self . after . clone () , after : self . before . clone () , } } # [doc = " Returns whether tokens are only inserted and not removed in this hunk."] pub fn is_pure_insertion (& self) -> bool { self . before . is_empty () } # [doc = " Returns whether tokens are only removed and not inserted in this hunk."] pub fn is_pure_removal (& self) -> bool { self . after . is_empty () } }
    };
}

impl_113!()