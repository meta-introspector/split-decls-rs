macro_rules! deps {
    () => {
        Pair!();
        LineIndex!();
        QueueableToken!();
    };
}

macro_rules! Pairs {
    () => {
        deps!();
        # [doc = " An iterator over [`Pair`]s. It is created by [`pest::state`] and [`Pair::into_inner`]."] # [doc = ""] # [doc = " [`Pair`]: struct.Pair.html"] # [doc = " [`pest::state`]: ../fn.state.html"] # [doc = " [`Pair::into_inner`]: struct.Pair.html#method.into_inner"] # [derive (Clone)] pub struct Pairs < 'i , R > { queue : Rc < Vec < QueueableToken < 'i , R > > > , input : & 'i str , start : usize , end : usize , pairs_count : usize , line_index : Rc < LineIndex > , }
    };
}

Pairs!();