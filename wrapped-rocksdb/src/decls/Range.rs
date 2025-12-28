macro_rules! Range {
    () => {
        # [doc = " A range of keys, `start_key` is included, but not `end_key`."] # [doc = ""] # [doc = " You should make sure `end_key` is not less than `start_key`."] pub struct Range < 'a > { start_key : & 'a [u8] , end_key : & 'a [u8] , }
    };
}

Range!()