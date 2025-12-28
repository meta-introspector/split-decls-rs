macro_rules! deps {
    () => {
        HomogeneousTuple!();
    };
}

macro_rules! Tuples {
    () => {
        deps!();
        # [doc = " An iterator that groups the items in tuples of a specific size."] # [doc = ""] # [doc = " See [`.tuples()`](crate::Itertools::tuples) for more information."] # [derive (Clone , Debug)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Tuples < I , T > where I : Iterator < Item = T :: Item > , T : HomogeneousTuple , { iter : Fuse < I > , buf : T :: Buffer , }
    };
}

Tuples!()