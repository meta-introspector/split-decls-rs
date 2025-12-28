macro_rules! deps {
    () => {
        Version!();
    };
}

macro_rules! Protocols {
    () => {
        deps!();
        # [doc = " An iterator over the list of protocols a version supports."] # [derive (Clone)] pub struct Protocols < 'a > { cur : * const * const c_char , _inner : & 'a Version , }
    };
}

Protocols!()