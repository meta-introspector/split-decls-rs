macro_rules! NeverToken {
    () => {
        # [doc = " This flavor doesn't need a token."] pub (crate) type NeverToken = () ;
    };
}

NeverToken!()