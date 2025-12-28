macro_rules! WhileSome {
    () => {
        # [doc = " An iterator adaptor that filters `Option<A>` iterator elements"] # [doc = " and produces `A`. Stops on the first `None` encountered."] # [doc = ""] # [doc = " See [`.while_some()`](crate::Itertools::while_some) for more information."] # [derive (Clone , Debug)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct WhileSome < I > { iter : I , }
    };
}

WhileSome!();