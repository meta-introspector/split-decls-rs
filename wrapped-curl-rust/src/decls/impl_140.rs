macro_rules! deps {
    () => {
        DetachGuard!();
        MultiError!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl DetachGuard { # [doc = " Detach the referenced easy handle from its multi handle manually."] # [doc = " Subsequent calls to this method will have no effect."] fn detach (& mut self) -> Result < () , MultiError > { if ! self . easy . is_null () { unsafe { cvt (curl_sys :: curl_multi_remove_handle (self . multi . handle , self . easy ,)) ? } self . easy = ptr :: null_mut () ; } Ok (()) } }
    };
}

impl_140!()