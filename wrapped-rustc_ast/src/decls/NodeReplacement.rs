macro_rules! deps {
    () => {
        NodeRange!();
        ParserReplacement!();
        AttrsTarget!();
    };
}

macro_rules! NodeReplacement {
    () => {
        deps!();
        # [doc = " See the comment on `ParserReplacement`."] pub type NodeReplacement = (NodeRange , Option < AttrsTarget >) ;
    };
}

NodeReplacement!()