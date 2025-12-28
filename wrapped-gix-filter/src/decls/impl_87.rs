macro_rules! deps {
    () => {
        Operation!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Operation { # [doc = " Return a string that identifies the operation. This happens to be the command-names used in long-running processes as well."] pub fn as_str (& self) -> & 'static str { match self { Operation :: Clean => "clean" , Operation :: Smudge => "smudge" , } } }
    };
}

impl_87!()