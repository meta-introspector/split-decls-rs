macro_rules! experimental {
    () => {
        macro_rules ! experimental { ($ attr : ident) => { concat ! ("the `#[" , stringify ! ($ attr) , "]` attribute is an experimental feature") } ; }
    };
}

experimental!()