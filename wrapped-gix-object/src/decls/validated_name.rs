macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! validated_name {
    () => {
        deps!();
        fn validated_name (name : & BStr) -> Result < & BStr , Error > { gix_validate :: tag :: name (name) ? ; if name [0] == b'-' { return Err (Error :: StartsWithDash) ; } Ok (name) }
    };
}

validated_name!();