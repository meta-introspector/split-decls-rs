macro_rules! deps {
    () => {
        PpAnn!();
    };
}

macro_rules! attribute_to_string {
    () => {
        deps!();
        pub fn attribute_to_string (ann : & dyn PpAnn , attr : & hir :: Attribute) -> String { to_string (ann , | s | s . print_attribute_as_style (attr , ast :: AttrStyle :: Outer)) }
    };
}

attribute_to_string!()