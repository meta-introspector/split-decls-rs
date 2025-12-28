macro_rules! Data {
    () => {
        # [doc = " Some space to keep a `FnOnce()` object on the stack."] type Data = [usize ; DATA_WORDS] ;
    };
}

Data!()