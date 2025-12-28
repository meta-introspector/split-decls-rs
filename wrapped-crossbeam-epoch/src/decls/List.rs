macro_rules! deps {
    () => {
        IsElement!();
        Atomic!();
        Entry!();
    };
}

macro_rules! List {
    () => {
        deps!();
        # [doc = " A lock-free, intrusive linked list of type `T`."] # [derive (Debug)] pub (crate) struct List < T , C : IsElement < T > = T > { # [doc = " The head of the linked list."] head : Atomic < Entry > , # [doc = " The phantom data for using `T` and `C`."] _marker : PhantomData < (T , C) > , }
    };
}

List!()