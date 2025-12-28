macro_rules! deps {
    () => {
        UniqueBy!();
    };
}

macro_rules! unique_by {
    () => {
        deps!();
        # [doc = " Create a new `UniqueBy` iterator."] pub fn unique_by < I , V , F > (iter : I , f : F) -> UniqueBy < I , V , F > where V : Eq + Hash , F : FnMut (& I :: Item) -> V , I : Iterator , { UniqueBy { iter , used : HashMap :: new () , f , } }
    };
}

unique_by!()