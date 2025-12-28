macro_rules! deps {
    () => {
        ChangeSet!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl ChangeSet { # [doc = " Checks if there is nothing to change (used to detect the `</>` tag)."] pub fn is_void (& self) -> bool { and ! (self . foreground . is_none () , self . background . is_none () , ! self . bold , ! self . dim , ! self . underline , ! self . italics , ! self . blink , ! self . strike , ! self . reverse , ! self . conceal ,) } }
    };
}

impl_50!();