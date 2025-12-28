macro_rules! deps {
    () => {
        StyledStr!();
        Styles!();
    };
}

macro_rules! start_error {
    () => {
        deps!();
        fn start_error (styled : & mut StyledStr , styles : & Styles) { use std :: fmt :: Write as _ ; let error = & styles . get_error () ; let _ = write ! (styled , "{error}error:{error:#} ") ; }
    };
}

start_error!()