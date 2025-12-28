macro_rules! AndThen {
    () => {
        # [doc = " Implementation of `Parser::and_then`"] pub struct AndThen < F , G > { f : F , g : G , }
    };
}

AndThen!()