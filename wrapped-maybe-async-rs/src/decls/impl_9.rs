macro_rules! deps {
    () => {
        AsyncAwaitRemoval!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl AsyncAwaitRemoval { pub fn remove_async_await (& mut self , item : TokenStream) -> TokenStream { let mut syntax_tree : File = syn :: parse (item . into ()) . unwrap () ; self . visit_file_mut (& mut syntax_tree) ; quote ! (# syntax_tree) } }
    };
}

impl_9!();