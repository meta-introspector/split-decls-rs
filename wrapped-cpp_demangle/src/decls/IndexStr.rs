macro_rules! IndexStr {
    () => {
        # [doc = " The `IndexStr` type allows us to take substrings from an original input and"] # [doc = " keep track of what index the substring is at in the original input."] # [derive (Clone , Copy , PartialEq , Eq)] pub struct IndexStr < 'a > { idx : usize , string : & 'a [u8] , }
    };
}

IndexStr!()