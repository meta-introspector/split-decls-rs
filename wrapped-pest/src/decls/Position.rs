macro_rules! Position {
    () => {
        # [doc = " A cursor position in a `&str` which provides useful methods to manually parse that string."] # [derive (Clone , Copy)] pub struct Position < 'i > { input : & 'i str , pos : usize , }
    };
}

Position!()