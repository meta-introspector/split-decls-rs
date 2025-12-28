macro_rules! deps {
    () => {
        Builder!();
        Config!();
        Compiler!();
        BuildError!();
        NFA!();
        PikeVM!();
    };
}

macro_rules! impl_537 {
    () => {
        deps!();
        impl Builder { # [doc = " Create a new PikeVM builder with its default configuration."] pub fn new () -> Builder { Builder { config : Config :: default () , # [cfg (feature = "syntax")] thompson : thompson :: Compiler :: new () , } } # [doc = " Build a `PikeVM` from the given pattern."] # [doc = ""] # [doc = " If there was a problem parsing or compiling the pattern, then an error"] # [doc = " is returned."] # [cfg (feature = "syntax")] pub fn build (& self , pattern : & str) -> Result < PikeVM , BuildError > { self . build_many (& [pattern]) } # [doc = " Build a `PikeVM` from the given patterns."] # [cfg (feature = "syntax")] pub fn build_many < P : AsRef < str > > (& self , patterns : & [P] ,) -> Result < PikeVM , BuildError > { let nfa = self . thompson . build_many (patterns) ? ; self . build_from_nfa (nfa) } # [doc = " Build a `PikeVM` directly from its NFA."] # [doc = ""] # [doc = " Note that when using this method, any configuration that applies to the"] # [doc = " construction of the NFA itself will of course be ignored, since the NFA"] # [doc = " given here is already built."] pub fn build_from_nfa (& self , nfa : NFA) -> Result < PikeVM , BuildError > { nfa . look_set_any () . available () . map_err (BuildError :: word) ? ; Ok (PikeVM { config : self . config . clone () , nfa }) } # [doc = " Apply the given `PikeVM` configuration options to this builder."] pub fn configure (& mut self , config : Config) -> & mut Builder { self . config = self . config . overwrite (config) ; self } # [doc = " Set the syntax configuration for this builder using"] # [doc = " [`syntax::Config`](crate::util::syntax::Config)."] # [doc = ""] # [doc = " This permits setting things like case insensitivity, Unicode and multi"] # [doc = " line mode."] # [doc = ""] # [doc = " These settings only apply when constructing a PikeVM directly from a"] # [doc = " pattern."] # [cfg (feature = "syntax")] pub fn syntax (& mut self , config : crate :: util :: syntax :: Config ,) -> & mut Builder { self . thompson . syntax (config) ; self } # [doc = " Set the Thompson NFA configuration for this builder using"] # [doc = " [`nfa::thompson::Config`](crate::nfa::thompson::Config)."] # [doc = ""] # [doc = " This permits setting things like if additional time should be spent"] # [doc = " shrinking the size of the NFA."] # [doc = ""] # [doc = " These settings only apply when constructing a PikeVM directly from a"] # [doc = " pattern."] # [cfg (feature = "syntax")] pub fn thompson (& mut self , config : thompson :: Config) -> & mut Builder { self . thompson . configure (config) ; self } }
    };
}

impl_537!()