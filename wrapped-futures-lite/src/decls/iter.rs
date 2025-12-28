macro_rules! deps {
    () => {
        StreamExt!();
        Iter!();
    };
}

macro_rules! iter {
    () => {
        deps!();
        # [doc = " Creates a stream from an iterator."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::stream::{self, StreamExt};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut s = stream::iter(vec![1, 2]);"] # [doc = ""] # [doc = " assert_eq!(s.next().await, Some(1));"] # [doc = " assert_eq!(s.next().await, Some(2));"] # [doc = " assert_eq!(s.next().await, None);"] # [doc = " # })"] # [doc = " ```"] pub fn iter < I : IntoIterator > (iter : I) -> Iter < I :: IntoIter > { Iter { iter : iter . into_iter () , } }
    };
}

iter!();