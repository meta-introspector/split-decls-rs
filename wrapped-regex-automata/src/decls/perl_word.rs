macro_rules! perl_word {
    () => {
        # [cfg (all (feature = "unicode-word-boundary" , not (all (feature = "syntax" , feature = "unicode-perl" ,)) ,))] pub (crate) mod perl_word ;
    };
}

perl_word!()