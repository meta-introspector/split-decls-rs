macro_rules! Or {
    () => {
        # [doc = " Implementation of `Parser::or`"] pub struct Or < F , G > { f : F , g : G , }
    };
}

Or!()