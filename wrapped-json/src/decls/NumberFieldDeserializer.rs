macro_rules! NumberFieldDeserializer {
    () => {
        # [cfg (feature = "arbitrary_precision")] struct NumberFieldDeserializer ;
    };
}

NumberFieldDeserializer!();