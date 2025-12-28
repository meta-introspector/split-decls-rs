macro_rules! is_non_enum {
    () => {
        # [doc = " Returns `true` if we know for sure that the given type is not an enum. Note that for cases where"] # [doc = " the type is generic, we can't be certain if it will be an enum so we have to assume that it is."] fn is_non_enum (t : Ty < '_ >) -> bool { ! t . is_enum () && ! t . has_param () }
    };
}

is_non_enum!()