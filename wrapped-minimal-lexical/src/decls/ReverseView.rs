macro_rules! ReverseView {
    () => {
        # [doc = " REVERSE VIEW"] # [doc = " Reverse, immutable view of a sequence."] pub struct ReverseView < 'a , T : 'a > { inner : & 'a [T] , }
    };
}

ReverseView!()