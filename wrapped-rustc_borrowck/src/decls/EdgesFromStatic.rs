macro_rules! EdgesFromStatic {
    () => {
        pub (crate) struct EdgesFromStatic { next_static_idx : usize , end_static_idx : usize , }
    };
}

EdgesFromStatic!();