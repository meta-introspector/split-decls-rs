macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! update {
    () => {
        deps!();
        # [doc = " Create a new `Update` iterator."] pub fn update < I , F > (iter : I , f : F) -> Update < I , F > where I : Iterator , F : FnMut (& mut I :: Item) , { Update { iter , f } }
    };
}

update!();