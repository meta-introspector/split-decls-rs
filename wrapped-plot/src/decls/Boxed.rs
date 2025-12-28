macro_rules! Boxed {
    () => {
        # [doc = " Whether the key is surrounded by a box or not"] # [allow (missing_docs)] # [derive (Clone , Copy)] pub enum Boxed { No , Yes , }
    };
}

Boxed!()