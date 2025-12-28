macro_rules! mockable_fn {
    () => {
        # [doc = " Performs transformations on a function to make it mockable"] fn mockable_fn (mut item_fn : ItemFn) -> ItemFn { demutify (& mut item_fn . sig . inputs) ; deimplify (& mut item_fn . sig . output) ; item_fn }
    };
}

mockable_fn!();