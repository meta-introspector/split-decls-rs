macro_rules! deps {
    () => {
        PostExpansionVisitor!();
    };
}

macro_rules! check_attribute {
    () => {
        deps!();
        pub fn check_attribute (attr : & ast :: Attribute , sess : & Session , features : & Features) { PostExpansionVisitor { sess , features } . visit_attribute (attr) }
    };
}

check_attribute!()