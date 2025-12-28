macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! KeyIterator {
    () => {
        deps!();
        # [doc = " An iterator of registry key names."] pub struct KeyIterator < 'a > { key : & 'a Key , range : core :: ops :: Range < usize > , name : Vec < u16 > , }
    };
}

KeyIterator!();