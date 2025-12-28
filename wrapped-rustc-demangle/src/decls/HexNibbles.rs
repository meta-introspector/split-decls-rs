macro_rules! HexNibbles {
    () => {
        # [doc = " Sequence of lowercase hexadecimal nibbles (`0-9a-f`), used by leaf consts."] struct HexNibbles < 's > { nibbles : & 's str , }
    };
}

HexNibbles!()