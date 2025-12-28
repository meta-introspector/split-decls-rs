macro_rules! use_as_display {
    () => {
        fn use_as_display (needs_as_display : bool) -> Option < TokenStream > { if needs_as_display { Some (quote ! { use :: thiserror ::# private :: AsDisplay as _ ; }) } else { None } }
    };
}

use_as_display!();