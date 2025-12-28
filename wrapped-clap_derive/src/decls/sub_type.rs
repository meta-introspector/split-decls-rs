macro_rules! sub_type {
    () => {
        pub (crate) fn sub_type (ty : & Type) -> Option < & Type > { subty_if (ty , | _ | true) }
    };
}

sub_type!();