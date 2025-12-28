macro_rules! MissingDoc {
    () => {
        # [derive (Default)] pub struct MissingDoc ;
    };
}

MissingDoc!();