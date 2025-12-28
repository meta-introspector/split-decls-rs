macro_rules! deps {
    () => {
        TakeWhileRef!();
    };
}

macro_rules! take_while_ref {
    () => {
        deps!();
        # [doc = " Create a new `TakeWhileRef` from a reference to cloneable iterator."] pub fn take_while_ref < I , F > (iter : & mut I , f : F) -> TakeWhileRef < '_ , I , F > where I : Iterator + Clone , { TakeWhileRef { iter , f } }
    };
}

take_while_ref!()