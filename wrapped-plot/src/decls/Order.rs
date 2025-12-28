macro_rules! Order {
    () => {
        # [doc = " Order of the elements of the key"] # [derive (Clone , Copy)] pub enum Order { # [doc = " Sample first, then text"] SampleText , # [doc = " Text first, then sample"] TextSample , }
    };
}

Order!()