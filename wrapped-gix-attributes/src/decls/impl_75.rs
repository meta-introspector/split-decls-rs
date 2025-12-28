macro_rules! deps {
    () => {
        Error!();
        AssignmentRef!();
        Iter!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < 'a > Iterator for Iter < 'a > { type Item = Result < AssignmentRef < 'a > , name :: Error > ; fn next (& mut self) -> Option < Self :: Item > { let attr = self . attrs . next () . filter (| a | ! a . is_empty ()) ? ; self . parse_attr (attr) . into () } }
    };
}

impl_75!();