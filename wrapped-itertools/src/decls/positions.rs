macro_rules! deps {
    () => {
        Positions!();
    };
}

macro_rules! positions {
    () => {
        deps!();
        # [doc = " Create a new `Positions` iterator."] pub fn positions < I , F > (iter : I , f : F) -> Positions < I , F > where I : Iterator , F : FnMut (I :: Item) -> bool , { let iter = iter . enumerate () ; Positions { iter , f } }
    };
}

positions!()