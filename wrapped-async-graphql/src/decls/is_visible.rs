macro_rules! deps {
    () => {
        Context!();
        MetaVisibleFn!();
    };
}

macro_rules! is_visible {
    () => {
        deps!();
        pub (crate) fn is_visible (ctx : & Context < '_ > , visible : & Option < MetaVisibleFn >) -> bool { match visible { Some (f) => f (ctx) , None => true , } }
    };
}

is_visible!();