macro_rules! deps {
    () => {
        Styles!();
        StyledStr!();
    };
}

macro_rules! try_help {
    () => {
        deps!();
        fn try_help (styled : & mut StyledStr , styles : & Styles , help : Option < & str >) { if let Some (help) = help { use std :: fmt :: Write as _ ; let literal = & styles . get_literal () ; let _ = write ! (styled , "\n\nFor more information, try '{literal}{help}{literal:#}'.\n" ,) ; } else { styled . push_str ("\n") ; } }
    };
}

try_help!();