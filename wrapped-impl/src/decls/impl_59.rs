macro_rules! deps {
    () => {
        Enum!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl Enum < '_ > { pub (crate) fn has_source (& self) -> bool { self . variants . iter () . any (| variant | variant . source_field () . is_some () || variant . attrs . transparent . is_some ()) } pub (crate) fn has_backtrace (& self) -> bool { self . variants . iter () . any (| variant | variant . backtrace_field () . is_some ()) } pub (crate) fn has_display (& self) -> bool { self . attrs . display . is_some () || self . attrs . transparent . is_some () || self . attrs . fmt . is_some () || self . variants . iter () . any (| variant | variant . attrs . display . is_some () || variant . attrs . fmt . is_some ()) || self . variants . iter () . all (| variant | variant . attrs . transparent . is_some ()) } }
    };
}

impl_59!();