macro_rules! deps {
    () => {
        DequeInner!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T , S : VecStorage < T > + ? Sized > Drop for DequeInner < T , S > { fn drop (& mut self) { unsafe { self . drop_contents () } } }
    };
}

impl_37!()