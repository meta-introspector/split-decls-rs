macro_rules! PostExpansionVisitor {
    () => {
        struct PostExpansionVisitor < 'a > { sess : & 'a Session , features : & 'a Features , }
    };
}

PostExpansionVisitor!()