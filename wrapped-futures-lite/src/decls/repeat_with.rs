macro_rules! deps {
    () => {
        RepeatWith!();
        StreamExt!();
    };
}

macro_rules! repeat_with {
    () => {
        deps!();
        # [doc = " Creates an infinite stream from a closure that generates items."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::stream::{self, StreamExt};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut s = stream::repeat_with(|| 7);"] # [doc = ""] # [doc = " assert_eq!(s.next().await, Some(7));"] # [doc = " assert_eq!(s.next().await, Some(7));"] # [doc = " # })"] # [doc = " ```"] pub fn repeat_with < T , F > (repeater : F) -> RepeatWith < F > where F : FnMut () -> T , { RepeatWith { f : repeater } }
    };
}

repeat_with!();