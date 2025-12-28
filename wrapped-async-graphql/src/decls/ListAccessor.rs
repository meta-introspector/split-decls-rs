macro_rules! ListAccessor {
    () => {
        # [doc = " A list accessor"] pub struct ListAccessor < 'a > (pub (crate) & 'a [Value]) ;
    };
}

ListAccessor!()