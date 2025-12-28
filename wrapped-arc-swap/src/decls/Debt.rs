macro_rules! Debt {
    () => {
        # [doc = " One debt slot."] # [doc = ""] # [doc = " It may contain an „owed“ reference count."] # [derive (Debug)] pub (crate) struct Debt (pub (crate) AtomicUsize) ;
    };
}

Debt!();