macro_rules! deps {
    () => {
        StreamExt!();
    };
}

macro_rules! unfold {
    () => {
        deps!();
        # [doc = " Creates a stream from a seed value and an async closure operating on it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::stream::{self, StreamExt};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let s = stream::unfold(0, |mut n| async move {"] # [doc = "     if n < 2 {"] # [doc = "         let m = n + 1;"] # [doc = "         Some((n, m))"] # [doc = "     } else {"] # [doc = "         None"] # [doc = "     }"] # [doc = " });"] # [doc = ""] # [doc = " let v: Vec<i32> = s.collect().await;"] # [doc = " assert_eq!(v, [0, 1]);"] # [doc = " # })"] # [doc = " ```"] pub fn unfold < T , F , Fut , Item > (seed : T , f : F) -> Unfold < T , F , Fut > where F : FnMut (T) -> Fut , Fut : Future < Output = Option < (Item , T) > > , { Unfold { f , state : Some (seed) , fut : None , } }
    };
}

unfold!()