macro_rules! deps {
    () => {
        Table!();
        TableIterMut!();
        Item!();
    };
}

macro_rules! impl_1100 {
    () => {
        deps!();
        impl < 'a , T : Item > IntoIterator for & 'a mut Table < T > { type Item = & 'a mut T ; type IntoIter = TableIterMut < 'a , T > ; fn into_iter (self) -> TableIterMut < 'a , T > { TableIterMut { iter : self . 0 . iter_mut () , } } }
    };
}

impl_1100!();