macro_rules! deps {
    () => {
        Visible!();
    };
}

macro_rules! visible_fn {
    () => {
        deps!();
        pub fn visible_fn (visible : & Option < Visible >) -> TokenStream { match visible { None | Some (Visible :: None) => quote ! { :: std :: option :: Option :: None } , Some (Visible :: HiddenAlways) => quote ! { :: std :: option :: Option :: Some (| _ | false) } , Some (Visible :: FnName (name)) => { quote ! { :: std :: option :: Option :: Some (# name) } } } }
    };
}

visible_fn!()