macro_rules! deps {
    () => {
        DFA!();
        Builder!();
        BuildError!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl DFA { # [doc = " Create a new Aho-Corasick DFA using the default configuration."] # [doc = ""] # [doc = " Use a [`Builder`] if you want to change the configuration."] pub fn new < I , P > (patterns : I) -> Result < DFA , BuildError > where I : IntoIterator < Item = P > , P : AsRef < [u8] > , { DFA :: builder () . build (patterns) } # [doc = " A convenience method for returning a new Aho-Corasick DFA builder."] # [doc = ""] # [doc = " This usually permits one to just import the `DFA` type."] pub fn builder () -> Builder { Builder :: new () } }
    };
}

impl_56!()