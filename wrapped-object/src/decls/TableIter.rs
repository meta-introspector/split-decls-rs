macro_rules! deps {
    () => {
        Table!();
    };
}

macro_rules! TableIter {
    () => {
        deps!();
        # [doc = " An iterator for non-deleted items in a [`Table`]."] # [derive (Debug)] pub struct TableIter < 'a , T > { iter : core :: slice :: Iter < 'a , T > , }
    };
}

TableIter!();