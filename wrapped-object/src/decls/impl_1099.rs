macro_rules! deps {
    () => {
        Item!();
        Table!();
        TableIter!();
    };
}

macro_rules! impl_1099 {
    () => {
        deps!();
        impl < 'a , T : Item > IntoIterator for & 'a Table < T > { type Item = & 'a T ; type IntoIter = TableIter < 'a , T > ; fn into_iter (self) -> TableIter < 'a , T > { TableIter { iter : self . 0 . iter () , } } }
    };
}

impl_1099!()