macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! TableIterMut {
    () => {
        deps!();
        # [doc = " An iterator for non-deleted items in a [`Table`]."] # [derive (Debug)] pub struct TableIterMut < 'a , T > { iter : core :: slice :: IterMut < 'a , T > , }
    };
}

TableIterMut!()