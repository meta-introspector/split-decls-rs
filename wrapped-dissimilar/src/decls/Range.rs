macro_rules! Range {
    () => {
        # [derive (Copy , Clone)] pub struct Range < 'a > { pub doc : & 'a [char] , pub offset : usize , pub len : usize , }
    };
}

Range!();