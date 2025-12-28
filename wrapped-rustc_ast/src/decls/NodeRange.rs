macro_rules! deps {
    () => {
        ParserRange!();
    };
}

macro_rules! NodeRange {
    () => {
        deps!();
        # [doc = " A token range within an individual AST node's (lazy) token stream, i.e."] # [doc = " relative to that node's first token. Distinct from `ParserRange` so the two"] # [doc = " kinds of range can't be mixed up."] # [derive (Clone , Debug)] pub struct NodeRange (pub Range < u32 >) ;
    };
}

NodeRange!();