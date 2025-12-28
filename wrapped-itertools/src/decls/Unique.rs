macro_rules! deps {
    () => {
        UniqueBy!();
    };
}

macro_rules! Unique {
    () => {
        deps!();
        # [doc = " An iterator adapter to filter out duplicate elements."] # [doc = ""] # [doc = " See [`.unique()`](crate::Itertools::unique) for more information."] # [derive (Clone)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Unique < I > where I : Iterator , I :: Item : Eq + Hash + Clone , { iter : UniqueBy < I , I :: Item , () > , }
    };
}

Unique!()