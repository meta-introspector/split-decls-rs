macro_rules! deps {
    () => {
        Mutex!();
        MutexGuard!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl < 'a , T : ? Sized + 'a > MutexGuard < 'a , T > { pub (super) fn unborrow (& mut self) { self . data = None ; } pub (super) fn reborrow (& mut self) { self . data = Some (self . lock . data . lock () . unwrap ()) ; } pub (super) fn rt (& self) -> & rt :: Mutex { & self . lock . object } }
    };
}

impl_301!()