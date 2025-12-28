macro_rules! deps {
    () => {
        Visitor!();
        Defaultness!();
    };
}

macro_rules! walk_defaultness {
    () => {
        deps!();
        pub fn walk_defaultness < 'v , V : Visitor < 'v > > (_ : & mut V , _ : & 'v Defaultness) -> V :: Result { V :: Result :: output () }
    };
}

walk_defaultness!()