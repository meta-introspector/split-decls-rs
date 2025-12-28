macro_rules! deps {
    () => {
        Item!();
        TableIterMut!();
    };
}

macro_rules! impl_1104 {
    () => {
        deps!();
        impl < 'a , T : Item > Iterator for TableIterMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < & 'a mut T > { self . iter . find (| item | ! item . is_deleted ()) } }
    };
}

impl_1104!();