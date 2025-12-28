macro_rules! impl_ {
    () => {
        # [cfg (not (feature = "cache-efficiency-debug"))] mod impl_ { # [doc = " The disabled, zero size do-nothing equivalent"] pub struct Debug ; impl Debug { # [doc = " Create a new instance"] # [inline] pub fn new (_owner : String) -> Self { Debug } # [doc = " noop"] pub fn put (& mut self) { } # [doc = " noop"] pub fn hit (& mut self) { } # [doc = " noop"] pub fn miss (& mut self) { } } }
    };
}

impl_!()