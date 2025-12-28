macro_rules! Item {
    () => {
        # [doc = " A set of tracked items allows to figure out their relations by figuring out their similarity."] pub (crate) struct Item < T > { # [doc = " The underlying raw change"] change : T , # [doc = " That slice into the backing for paths."] path : Range < usize > , # [doc = " If true, this item was already emitted, i.e. seen by the caller."] emitted : bool , }
    };
}

Item!();