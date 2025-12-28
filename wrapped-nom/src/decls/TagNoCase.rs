macro_rules! deps {
    () => {
        Tag!();
    };
}

macro_rules! TagNoCase {
    () => {
        deps!();
        # [doc = " Case insensitive Tag implementation"] pub struct TagNoCase < T , E > { tag : T , e : PhantomData < E > , }
    };
}

TagNoCase!();