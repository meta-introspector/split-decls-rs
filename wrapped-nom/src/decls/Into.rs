macro_rules! Into {
    () => {
        # [doc = " Implementation of `Parser::into`"] pub struct Into < F , O2 , E2 > { f : F , phantom_out2 : core :: marker :: PhantomData < O2 > , phantom_err2 : core :: marker :: PhantomData < E2 > , }
    };
}

Into!()