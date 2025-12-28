macro_rules! deps {
    () => {
        CharLit!();
    };
}

macro_rules! macro_166 {
    () => {
        deps!();
        helper ! (impl_for_specific_lit , crate :: CharLit < String >, Char , CharLit) ;
    };
}

macro_166!();