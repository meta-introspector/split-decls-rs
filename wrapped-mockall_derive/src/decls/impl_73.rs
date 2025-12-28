macro_rules! deps {
    () => {
        AttrFormatter!();
        MockItemTraitImpl!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl ToTokens for MockItemTraitImpl { fn to_tokens (& self , tokens : & mut TokenStream) { let mod_attrs = AttrFormatter :: new (& self . attrs) . async_trait (false) . doc (false) . format () ; let struct_attrs = AttrFormatter :: new (& self . attrs) . async_trait (false) . doc (false) . must_use (false) . format () ; let impl_attrs = AttrFormatter :: new (& self . attrs) . async_trait (false) . doc (false) . format () ; let struct_name = & self . name ; let (ig , tg , wc) = self . generics . split_for_impl () ; let modname = & self . modname ; let method_checkpoints = self . methods . checkpoints () ; let mut default_inits = self . methods . default_inits () ; default_inits . extend (self . phantom_default_inits ()) ; let mut field_definitions = self . methods . field_definitions (modname) ; field_definitions . extend (self . phantom_fields ()) ; let priv_mods = self . methods . priv_mods () ; quote ! (# [allow (non_snake_case)] # [allow (missing_docs)] # (# mod_attrs) * pub mod # modname { use super ::*; # (# priv_mods) * } # [allow (non_camel_case_types)] # [allow (non_snake_case)] # [allow (missing_docs)] # (# struct_attrs) * struct # struct_name # ig # wc { # (# field_definitions) ,* } # (# impl_attrs) * impl # ig :: std :: default :: Default for # struct_name # tg # wc { fn default () -> Self { Self { # (# default_inits) ,* } } } # (# impl_attrs) * impl # ig # struct_name # tg # wc { # [doc = " Validate that all current expectations for all methods have"] # [doc = " been satisfied, and discard them."] pub fn checkpoint (& mut self) { # (# method_checkpoints) * } }) . to_tokens (tokens) ; } }
    };
}

impl_73!()