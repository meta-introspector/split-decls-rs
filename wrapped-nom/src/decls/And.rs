macro_rules! And {
    () => {
        # [doc = " Implementation of `Parser::and`"] pub struct And < F , G > { f : F , g : G , }
    };
}

And!();