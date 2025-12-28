macro_rules! deps {
    () => {
        StreamExt!();
    };
}

macro_rules! try_unfold {
    () => {
        deps!();
        # [doc = " Creates a stream from a seed value and a fallible async closure operating on it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::stream::{self, StreamExt};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let s = stream::try_unfold(0, |mut n| async move {"] # [doc = "     if n < 2 {"] # [doc = "         let m = n + 1;"] # [doc = "         Ok(Some((n, m)))"] # [doc = "     } else {"] # [doc = "         std::io::Result::Ok(None)"] # [doc = "     }"] # [doc = " });"] # [doc = ""] # [doc = " let v: Vec<i32> = s.try_collect().await?;"] # [doc = " assert_eq!(v, [0, 1]);"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub fn try_unfold < T , E , F , Fut , Item > (init : T , f : F) -> TryUnfold < T , F , Fut > where F : FnMut (T) -> Fut , Fut : Future < Output = Result < Option < (Item , T) > , E > > , { TryUnfold { f , state : Some (init) , fut : None , } }
    };
}

try_unfold!();