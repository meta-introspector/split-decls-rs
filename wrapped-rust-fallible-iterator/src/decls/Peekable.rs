macro_rules! deps {
    () => {
        FallibleIterator!();
    };
}

macro_rules! Peekable {
    () => {
        deps!();
        # [doc = " An iterator which can look at the next element without consuming it."] # [derive (Clone , Debug)] pub struct Peekable < I : FallibleIterator > { it : I , next : Option < I :: Item > , }
    };
}

Peekable!();