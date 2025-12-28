macro_rules! deps {
    () => {
        Deprecation!();
    };
}

macro_rules! write_deprecated {
    () => {
        deps!();
        fn write_deprecated (sdl : & mut String , deprecation : & Deprecation) { if let Deprecation :: Deprecated { reason } = deprecation { let _ = match reason { Some (reason) => write ! (sdl , " @deprecated(reason: \"{}\")" , escape_string (reason)) . ok () , None => write ! (sdl , " @deprecated") . ok () , } ; } }
    };
}

write_deprecated!();