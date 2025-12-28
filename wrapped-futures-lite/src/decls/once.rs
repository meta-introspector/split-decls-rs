macro_rules! deps {
    () => {
        StreamExt!();
    };
}

macro_rules! once {
    () => {
        deps!();
        # [doc = " Creates a stream that yields a single item."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::stream::{self, StreamExt};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut s = stream::once(7);"] # [doc = ""] # [doc = " assert_eq!(s.next().await, Some(7));"] # [doc = " assert_eq!(s.next().await, None);"] # [doc = " # })"] # [doc = " ```"] pub fn once < T > (t : T) -> Once < T > { Once { value : Some (t) } }
    };
}

once!();