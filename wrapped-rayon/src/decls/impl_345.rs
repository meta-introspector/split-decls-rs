macro_rules! deps {
    () => {
        CollectResult!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl < 'c , T > CollectResult < 'c , T > { # [doc = " The current length of the collect result"] pub (super) fn len (& self) -> usize { self . initialized_len } # [doc = " Release ownership of the slice of elements, and return the length"] pub (super) fn release_ownership (mut self) -> usize { let ret = self . initialized_len ; self . initialized_len = 0 ; ret } }
    };
}

impl_345!()