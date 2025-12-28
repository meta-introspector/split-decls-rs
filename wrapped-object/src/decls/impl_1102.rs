macro_rules! deps {
    () => {
        Item!();
        TableIter!();
    };
}

macro_rules! impl_1102 {
    () => {
        deps!();
        impl < 'a , T : Item > Iterator for TableIter < 'a , T > { type Item = & 'a T ; fn next (& mut self) -> Option < & 'a T > { self . iter . find (| item | ! item . is_deleted ()) } }
    };
}

impl_1102!()