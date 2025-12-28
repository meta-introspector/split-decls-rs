macro_rules! deps {
    () => {
        Deferred!();
    };
}

macro_rules! Bag {
    () => {
        deps!();
        # [doc = " A bag of deferred functions."] pub (crate) struct Bag { # [doc = " Stashed objects."] deferreds : [Deferred ; MAX_OBJECTS] , len : usize , }
    };
}

Bag!();