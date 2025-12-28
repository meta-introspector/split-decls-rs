macro_rules! semi_for {
    () => {
        # [doc = " Return a semicolon token if necessary after the struct definition"] pub fn semi_for (f : & Fields) -> TokenStream2 { if let Fields :: Unnamed (..) = * f { quote ! (;) } else { quote ! () } }
    };
}

semi_for!()