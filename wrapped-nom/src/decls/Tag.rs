macro_rules! Tag {
    () => {
        # [doc = " Tag implementation"] pub struct Tag < T , E > { tag : T , e : PhantomData < E > , }
    };
}

Tag!()