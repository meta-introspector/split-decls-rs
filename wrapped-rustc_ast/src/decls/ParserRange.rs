macro_rules! ParserRange {
    () => {
        # [doc = " A token range within a `Parser`'s full token stream."] # [derive (Clone , Debug)] pub struct ParserRange (pub Range < u32 >) ;
    };
}

ParserRange!();