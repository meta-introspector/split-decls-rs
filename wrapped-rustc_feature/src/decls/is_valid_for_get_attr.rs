macro_rules! is_valid_for_get_attr {
    () => {
        pub fn is_valid_for_get_attr (name : Symbol) -> bool { BUILTIN_ATTRIBUTE_MAP . get (& name) . is_some_and (| attr | match attr . duplicates { WarnFollowing | ErrorFollowing | ErrorPreceding | FutureWarnFollowing | FutureWarnPreceding => true , DuplicatesOk | WarnFollowingWordOnly => false , }) }
    };
}

is_valid_for_get_attr!();