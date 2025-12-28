macro_rules! deps {
    () => {
        ParserReplacement!();
        AttrsTarget!();
        NodeRange!();
    };
}

macro_rules! NodeReplacement {
    () => {
        deps!();
        # [doc = " See the comment on `ParserReplacement`."] pub type NodeReplacement = (NodeRange , Option < AttrsTarget >) ;
    };
}

NodeReplacement!();