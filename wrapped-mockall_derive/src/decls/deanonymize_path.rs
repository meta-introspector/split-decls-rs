macro_rules! deps {
    () => {
        Methods!();
    };
}

macro_rules! deanonymize_path {
    () => {
        deps!();
        fn deanonymize_path (path : & mut Path) { for seg in path . segments . iter_mut () { match & mut seg . arguments { PathArguments :: None => () , PathArguments :: AngleBracketed (abga) => { for ga in abga . args . iter_mut () { if let GenericArgument :: Lifetime (lt) = ga { deanonymize_lifetime (lt) } } } , _ => compile_error (seg . arguments . span () , "Methods returning functions are TODO") , } } }
    };
}

deanonymize_path!()