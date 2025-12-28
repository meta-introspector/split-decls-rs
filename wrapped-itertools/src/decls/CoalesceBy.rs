macro_rules! deps {
    () => {
        CountItem!();
    };
}

macro_rules! CoalesceBy {
    () => {
        deps!();
        # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct CoalesceBy < I , F , C > where I : Iterator , C : CountItem < I :: Item > , { iter : I , # [doc = " `last` is `None` while no item have been taken out of `iter` (at definition)."] # [doc = " Then `last` will be `Some(Some(item))` until `iter` is exhausted,"] # [doc = " in which case `last` will be `Some(None)`."] last : Option < Option < C :: CItem > > , f : F , }
    };
}

CoalesceBy!();