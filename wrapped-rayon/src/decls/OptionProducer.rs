macro_rules! OptionProducer {
    () => {
        # [doc = " Private producer for an option"] struct OptionProducer < T : Send > { opt : Option < T > , }
    };
}

OptionProducer!()