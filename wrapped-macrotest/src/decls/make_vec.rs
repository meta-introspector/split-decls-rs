macro_rules! make_vec {
    () => {
        pub fn make_vec () -> Vec < String > { let mut rustflags = Vec :: new () ; for & lint in IGNORED_LINTS { rustflags . push ("-A" . to_owned ()) ; rustflags . push (lint . to_owned ()) ; } rustflags }
    };
}

make_vec!();