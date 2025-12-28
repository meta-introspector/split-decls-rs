macro_rules! Batching {
    () => {
        # [doc = " A “meta iterator adaptor”. Its closure receives a reference to the iterator"] # [doc = " and may pick off as many elements as it likes, to produce the next iterator element."] # [doc = ""] # [doc = " Iterator element type is `X` if the return type of `F` is `Option<X>`."] # [doc = ""] # [doc = " See [`.batching()`](crate::Itertools::batching) for more information."] # [derive (Clone)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Batching < I , F > { f : F , iter : I , }
    };
}

Batching!();