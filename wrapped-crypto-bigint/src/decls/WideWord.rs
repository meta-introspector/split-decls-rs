macro_rules! deps {
    () => {
        Word!();
    };
}

macro_rules! WideWord {
    () => {
        deps!();
        # [doc = " Wide integer type: double the width of [`Word`]."] # [cfg (target_pointer_width = "64")] pub type WideWord = u128 ;
    };
}

WideWord!()