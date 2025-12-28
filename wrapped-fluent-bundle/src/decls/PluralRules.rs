macro_rules! PluralRules {
    () => {
        pub struct PluralRules (pub IntlPluralRules) ;
    };
}

PluralRules!();