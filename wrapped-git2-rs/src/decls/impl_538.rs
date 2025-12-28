macro_rules! deps {
    () => {
        PackBuilder!();
    };
}

macro_rules! impl_538 {
    () => {
        deps!();
        impl < 'repo > Drop for PackBuilder < 'repo > { fn drop (& mut self) { unsafe { raw :: git_packbuilder_set_callbacks (self . raw , None , ptr :: null_mut ()) ; raw :: git_packbuilder_free (self . raw) ; } } }
    };
}

impl_538!()