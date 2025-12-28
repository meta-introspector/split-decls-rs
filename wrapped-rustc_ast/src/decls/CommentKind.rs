macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! CommentKind {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub enum CommentKind { Line , Block , }
    };
}

CommentKind!();