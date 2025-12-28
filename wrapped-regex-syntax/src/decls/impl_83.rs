macro_rules! deps {
    () => {
        ClassUnicodeKind!();
        ClassUnicodeOpKind!();
        ClassUnicode!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl ClassUnicode { # [doc = " Returns true if this class has been negated."] # [doc = ""] # [doc = " Note that this takes the Unicode op into account, if it's present."] # [doc = " e.g., `is_negated` for `\\P{scx!=Katakana}` will return `false`."] pub fn is_negated (& self) -> bool { match self . kind { ClassUnicodeKind :: NamedValue { op : ClassUnicodeOpKind :: NotEqual , .. } => ! self . negated , _ => self . negated , } } }
    };
}

impl_83!()