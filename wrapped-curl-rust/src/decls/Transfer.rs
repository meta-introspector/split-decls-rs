macro_rules! deps {
    () => {
        Callbacks!();
        Easy!();
    };
}

macro_rules! Transfer {
    () => {
        deps!();
        # [doc = " A scoped transfer of information which borrows an `Easy` and allows"] # [doc = " referencing stack-local data of the lifetime `'data`."] # [doc = ""] # [doc = " Usage of `Easy` requires the `'static` and `Send` bounds on all callbacks"] # [doc = " registered, but that's not often wanted if all you need is to collect a"] # [doc = " bunch of data in memory to a vector, for example. The `Transfer` structure,"] # [doc = " created by the `Easy::transfer` method, is used for this sort of request."] # [doc = ""] # [doc = " The callbacks attached to a `Transfer` are only active for that one transfer"] # [doc = " object, and they allow to elide both the `Send` and `'static` bounds to"] # [doc = " close over stack-local information."] pub struct Transfer < 'easy , 'data > { easy : & 'easy mut Easy , data : Box < Callbacks < 'data > > , }
    };
}

Transfer!()