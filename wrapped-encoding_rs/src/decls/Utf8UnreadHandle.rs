macro_rules! deps {
    () => {
        Utf8Source!();
    };
}

macro_rules! Utf8UnreadHandle {
    () => {
        deps!();
        pub struct Utf8UnreadHandle < 'a , 'b > where 'b : 'a , { source : & 'a mut Utf8Source < 'b > , old_pos : usize , }
    };
}

Utf8UnreadHandle!()