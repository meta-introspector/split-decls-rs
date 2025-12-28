macro_rules! UniqueBy {
    () => {
        # [doc = " An iterator adapter to filter out duplicate elements."] # [doc = ""] # [doc = " See [`.unique_by()`](crate::Itertools::unique) for more information."] # [derive (Clone)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct UniqueBy < I : Iterator , V , F > { iter : I , used : HashMap < V , () > , f : F , }
    };
}

UniqueBy!()