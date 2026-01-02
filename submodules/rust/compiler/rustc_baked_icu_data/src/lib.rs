mkitem!{mkstruct!{pub struct BakedDataProvider ;}}
mkitem!{include ! ("data/mod.rs") ;}
mkitem!{const _ : () = { impl_data_provider ! (BakedDataProvider) ; } ;}

macro_rules! baked_data_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function baked_data_provider in module {}", module_path!());
    };
}

mkfn!{
    baked_data_provider_introspect!();
    pub const fn baked_data_provider () -> BakedDataProvider { BakedDataProvider }
}
mkmod!{supported_locales, { 
                getname!(supported_locales);
                getsrc!(supported_locales);
                getpath!(supported_locales);
                get_deps!(supported_locales);
                get_crates!(supported_locales);
                mkinclude!(supported_locales);
                mkitem!{pub const EN : icu_locale :: Locale = icu_locale :: locale ! ("en") ;}
mkitem!{pub const ES : icu_locale :: Locale = icu_locale :: locale ! ("es") ;}
mkitem!{pub const FR : icu_locale :: Locale = icu_locale :: locale ! ("fr") ;}
mkitem!{pub const IT : icu_locale :: Locale = icu_locale :: locale ! ("it") ;}
mkitem!{pub const JA : icu_locale :: Locale = icu_locale :: locale ! ("ja") ;}
mkitem!{pub const PT : icu_locale :: Locale = icu_locale :: locale ! ("pt") ;}
mkitem!{pub const RU : icu_locale :: Locale = icu_locale :: locale ! ("ru") ;}
mkitem!{pub const TR : icu_locale :: Locale = icu_locale :: locale ! ("tr") ;}
mkitem!{pub const ZH_HANS : icu_locale :: Locale = icu_locale :: locale ! ("zh-Hans") ;}
mkitem!{pub const ZH_HANT : icu_locale :: Locale = icu_locale :: locale ! ("zh-Hant") ;} 
            }}