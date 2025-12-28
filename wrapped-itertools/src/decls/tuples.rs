macro_rules! deps {
    () => {
        HomogeneousTuple!();
        Tuples!();
    };
}

macro_rules! tuples {
    () => {
        deps!();
        # [doc = " Create a new tuples iterator."] pub fn tuples < I , T > (iter : I) -> Tuples < I , T > where I : Iterator < Item = T :: Item > , T : HomogeneousTuple , { Tuples { iter : iter . fuse () , buf : Default :: default () , } }
    };
}

tuples!()