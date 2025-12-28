macro_rules! deps {
    () => {
        Shared!();
        WeakShared!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < Fut : Future > WeakShared < Fut > { # [doc = " Attempts to upgrade this [`WeakShared`] into a [`Shared`]."] # [doc = ""] # [doc = " Returns [`None`] if all clones of the [`Shared`] have been dropped or polled"] # [doc = " to completion."] pub fn upgrade (& self) -> Option < Shared < Fut > > { Some (Shared { inner : Some (self . 0 . upgrade () ?) , waker_key : NULL_WAKER_KEY }) } }
    };
}

impl_106!();