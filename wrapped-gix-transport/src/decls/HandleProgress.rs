macro_rules! HandleProgress {
    () => {
        # [doc = " A function `f(is_error, text)` receiving progress or error information."] pub type HandleProgress < 'a > = Box < dyn FnMut (bool , & [u8]) -> ProgressAction + 'a > ;
    };
}

HandleProgress!();