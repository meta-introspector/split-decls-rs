macro_rules! deps {
    () => {
        Epoch!();
        SealedBag!();
        Bag!();
        Deferred!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl Bag { # [doc = " Returns a new, empty bag."] pub (crate) fn new () -> Self { Self :: default () } # [doc = " Returns `true` if the bag is empty."] pub (crate) fn is_empty (& self) -> bool { self . len == 0 } # [doc = " Attempts to insert a deferred function into the bag."] # [doc = ""] # [doc = " Returns `Ok(())` if successful, and `Err(deferred)` for the given `deferred` if the bag is"] # [doc = " full."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " It should be safe for another thread to execute the given function."] pub (crate) unsafe fn try_push (& mut self , deferred : Deferred) -> Result < () , Deferred > { if self . len < MAX_OBJECTS { self . deferreds [self . len] = deferred ; self . len += 1 ; Ok (()) } else { Err (deferred) } } # [doc = " Seals the bag with the given epoch."] fn seal (self , epoch : Epoch) -> SealedBag { SealedBag { epoch , _bag : self } } }
    };
}

impl_103!()