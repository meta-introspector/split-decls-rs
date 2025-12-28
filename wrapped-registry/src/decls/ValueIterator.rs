macro_rules! deps {
    () => {
        Key!();
        Data!();
    };
}

macro_rules! ValueIterator {
    () => {
        deps!();
        # [doc = " An iterator of registry values."] pub struct ValueIterator < 'a > { key : & 'a Key , range : core :: ops :: Range < usize > , name : Vec < u16 > , data : Data , }
    };
}

ValueIterator!();