macro_rules! experimental {
    () => {
        # [allow (unused_macros)] macro_rules ! experimental { ($ attr : ident) => { concat ! ("the `#[" , stringify ! ($ attr) , "]` attribute is an experimental feature") } ; }
    };
}

experimental!()