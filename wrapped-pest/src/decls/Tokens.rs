macro_rules! deps {
    () => {
        QueueableToken!();
        Token!();
    };
}

macro_rules! Tokens {
    () => {
        deps!();
        # [doc = " An iterator over [`Token`]s. It is created by [`Pair::tokens`] and [`Pairs::tokens`]."] # [doc = ""] # [doc = " [`Token`]: ../enum.Token.html"] # [doc = " [`Pair::tokens`]: struct.Pair.html#method.tokens"] # [doc = " [`Pairs::tokens`]: struct.Pairs.html#method.tokens"] # [derive (Clone)] pub struct Tokens < 'i , R > { queue : Rc < Vec < QueueableToken < 'i , R > > > , input : & 'i str , start : usize , end : usize , }
    };
}

Tokens!()