macro_rules! deps {
    () => {
        Comment!();
        CommentDef!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < S > From < CommentDef < S > > for Comment < S > { fn from (input : CommentDef < S >) -> Self { match input { CommentDef :: Single { content } => Self { content : vec ! [content] , } , CommentDef :: Multi { content } => Self { content } , } } }
    };
}

impl_1!();