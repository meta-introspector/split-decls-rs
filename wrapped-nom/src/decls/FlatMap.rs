macro_rules! FlatMap {
    () => {
        # [doc = " Implementation of `Parser::flat_map`"] pub struct FlatMap < F , G > { f : F , g : G , }
    };
}

FlatMap!();