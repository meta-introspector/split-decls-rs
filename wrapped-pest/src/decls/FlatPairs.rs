macro_rules! deps {
    () => {
        Pair!();
        QueueableToken!();
        LineIndex!();
    };
}

macro_rules! FlatPairs {
    () => {
        deps!();
        # [doc = " An iterator over [`Pair`]s. It is created by [`Pairs::flatten`]."] # [doc = ""] # [doc = " [`Pair`]: struct.Pair.html"] # [doc = " [`Pairs::flatten`]: struct.Pairs.html#method.flatten"] pub struct FlatPairs < 'i , R > { queue : Rc < Vec < QueueableToken < 'i , R > > > , input : & 'i str , start : usize , end : usize , line_index : Rc < LineIndex > , }
    };
}

FlatPairs!()