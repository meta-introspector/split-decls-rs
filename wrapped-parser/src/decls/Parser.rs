macro_rules! deps {
    () => {
        Event!();
        Input!();
    };
}

macro_rules! Parser {
    () => {
        deps!();
        # [doc = " `Parser` struct provides the low-level API for"] # [doc = " navigating through the stream of tokens and"] # [doc = " constructing the parse tree. The actual parsing"] # [doc = " happens in the [`grammar`](super::grammar) module."] # [doc = ""] # [doc = " However, the result of this `Parser` is not a real"] # [doc = " tree, but rather a flat stream of events of the form"] # [doc = " \"start expression, consume number literal,"] # [doc = " finish expression\". See `Event` docs for more."] pub (crate) struct Parser < 't > { inp : & 't Input , pos : usize , events : Vec < Event > , steps : Cell < u32 > , edition : Edition , }
    };
}

Parser!()