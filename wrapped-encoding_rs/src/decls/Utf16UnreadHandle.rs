macro_rules! deps {
    () => {
        Utf16Source!();
    };
}

macro_rules! Utf16UnreadHandle {
    () => {
        deps!();
        pub struct Utf16UnreadHandle < 'a , 'b > where 'b : 'a , { source : & 'a mut Utf16Source < 'b > , old_pos : usize , }
    };
}

Utf16UnreadHandle!()