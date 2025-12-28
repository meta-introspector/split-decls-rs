macro_rules! deps {
    () => {
        QueueableToken!();
        Token!();
        LineIndex!();
    };
}

macro_rules! Pair {
    () => {
        deps!();
        # [doc = " A matching pair of [`Token`]s and everything between them."] # [doc = ""] # [doc = " A matching `Token` pair is formed by a `Token::Start` and a subsequent `Token::End` with the"] # [doc = " same `Rule`, with the condition that all `Token`s between them can form such pairs as well."] # [doc = " This is similar to the [brace matching problem](https://en.wikipedia.org/wiki/Brace_matching) in"] # [doc = " editors."] # [doc = ""] # [doc = " [`Token`]: ../enum.Token.html"] # [derive (Clone)] pub struct Pair < 'i , R > { queue : Rc < Vec < QueueableToken < 'i , R > > > , input : & 'i str , # [doc = " Token index into `queue`."] start : usize , line_index : Rc < LineIndex > , }
    };
}

Pair!();