macro_rules! deps {
    () => {
        Pair!();
    };
}

macro_rules! Span {
    () => {
        deps!();
        # [doc = " A span over a `&str`. It is created from either [two `Position`s] or from a [`Pair`]."] # [doc = ""] # [doc = " [two `Position`s]: struct.Position.html#method.span"] # [doc = " [`Pair`]: ./iterators/struct.Pair.html#method.as_span"] # [derive (Clone , Copy)] pub struct Span < 'i > { input : & 'i str , start : usize , end : usize , }
    };
}

Span!()