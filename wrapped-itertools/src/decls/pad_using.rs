macro_rules! deps {
    () => {
        PadUsing!();
    };
}

macro_rules! pad_using {
    () => {
        deps!();
        # [doc = " Create a new `PadUsing` iterator."] pub fn pad_using < I , F > (iter : I , min : usize , filler : F) -> PadUsing < I , F > where I : Iterator , F : FnMut (usize) -> I :: Item , { PadUsing { iter : iter . fuse () , min , pos : 0 , filler , } }
    };
}

pad_using!()