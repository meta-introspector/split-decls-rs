macro_rules! apply {
    () => {
        fn apply < T > (update_op : impl Fn (& mut T)) -> impl Fn (T) -> T { move | mut item | { update_op (& mut item) ; item } }
    };
}

apply!()