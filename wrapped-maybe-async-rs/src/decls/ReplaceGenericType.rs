macro_rules! ReplaceGenericType {
    () => {
        pub struct ReplaceGenericType < 'a > { generic_type : & 'a str , arg_type : & 'a PathSegment , }
    };
}

ReplaceGenericType!();