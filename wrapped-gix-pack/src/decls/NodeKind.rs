macro_rules! NodeKind {
    () => {
        # [doc = " Identify what kind of node we have last seen"] enum NodeKind { Root , Child , }
    };
}

NodeKind!();