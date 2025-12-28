macro_rules! AttributePlace {
    () => {
        # [derive (Copy , Clone)] pub (crate) enum AttributePlace { ReturnValue , Argument (u32) , Function , }
    };
}

AttributePlace!()