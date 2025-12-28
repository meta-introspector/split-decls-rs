macro_rules! abort_on_panic {
    () => {
        # [doc = " Calls a function and aborts if it panics."] # [doc = ""] # [doc = " This is useful in unsafe code where we can't recover from panics."] # [cfg (feature = "default")] # [inline] pub fn abort_on_panic < T > (f : impl FnOnce () -> T) -> T { struct Bomb ; impl Drop for Bomb { fn drop (& mut self) { std :: process :: abort () ; } } let bomb = Bomb ; let t = f () ; std :: mem :: forget (bomb) ; t }
    };
}

abort_on_panic!();