macro_rules! deps {
    () => {
        Atomic!();
        Pointer!();
        Shared!();
        Entry!();
        IsElement!();
        Guard!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " An iterator used for retrieving values from the list."] pub (crate) struct Iter < 'g , T , C : IsElement < T > > { # [doc = " The guard that protects the iteration."] guard : & 'g Guard , # [doc = " Pointer from the predecessor to the current entry."] pred : & 'g Atomic < Entry > , # [doc = " The current entry."] curr : Shared < 'g , Entry > , # [doc = " The list head, needed for restarting iteration."] head : & 'g Atomic < Entry > , # [doc = " Logically, we store a borrow of an instance of `T` and"] # [doc = " use the type information from `C`."] _marker : PhantomData < (& 'g T , C) > , }
    };
}

Iter!();