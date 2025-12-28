macro_rules! deps {
    () => {
        BuiltinDriver!();
    };
}

macro_rules! impl_0 {
    () => {
        deps!();
        impl BuiltinDriver { # [doc = " Return the name of this instance."] pub fn as_str (& self) -> & str { match self { BuiltinDriver :: Text => "text" , BuiltinDriver :: Binary => "binary" , BuiltinDriver :: Union => "union" , } } # [doc = " Get all available built-in drivers."] pub fn all () -> & 'static [Self] { & [BuiltinDriver :: Text , BuiltinDriver :: Binary , BuiltinDriver :: Union] } # [doc = " Try to match one of our variants to `name`, case-sensitive, and return its instance."] pub fn by_name (name : & str) -> Option < Self > { Self :: all () . iter () . find (| variant | variant . as_str () == name) . copied () } }
    };
}

impl_0!();