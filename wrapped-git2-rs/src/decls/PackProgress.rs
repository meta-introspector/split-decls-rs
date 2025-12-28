macro_rules! deps {
    () => {
        PackBuilderStage!();
    };
}

macro_rules! PackProgress {
    () => {
        deps!();
        # [doc = " Callback for pack progress"] # [doc = ""] # [doc = " Be aware that this is called inline with pack building operations,"] # [doc = " so performance may be affected."] # [doc = ""] # [doc = " Parameters:"] # [doc = " * stage"] # [doc = " * current"] # [doc = " * total"] pub type PackProgress < 'a > = dyn FnMut (PackBuilderStage , usize , usize) + 'a ;
    };
}

PackProgress!()