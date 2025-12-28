macro_rules! deps {
    () => {
        Reflog!();
    };
}

macro_rules! ReflogIter {
    () => {
        deps!();
        # [doc = " An iterator over the entries inside of a reflog."] pub struct ReflogIter < 'reflog > { range : Range < usize > , reflog : & 'reflog Reflog , }
    };
}

ReflogIter!()