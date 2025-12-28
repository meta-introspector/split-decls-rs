macro_rules! deps {
    () => {
        StyledStr!();
    };
}

macro_rules! put_usage {
    () => {
        deps!();
        fn put_usage (styled : & mut StyledStr , usage : & StyledStr) { styled . push_str ("\n\n") ; styled . push_styled (usage) ; }
    };
}

put_usage!();