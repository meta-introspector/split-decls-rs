macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! ParserIterator {
    () => {
        deps!();
        # [doc = " Main structure associated to the [iterator] function."] pub struct ParserIterator < I , E , F > { iterator : F , input : I , state : Option < State < E > > , }
    };
}

ParserIterator!()