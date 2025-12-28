macro_rules! perl_word {
    () => {
        # [cfg (feature = "unicode-perl")] pub mod perl_word ;
    };
}

perl_word!();