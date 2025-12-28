macro_rules! Update {
    () => {
        # [doc = " An iterator adapter to apply a mutating function to each element before yielding it."] # [doc = ""] # [doc = " See [`.update()`](crate::Itertools::update) for more information."] # [derive (Clone)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Update < I , F > { iter : I , f : F , }
    };
}

Update!();