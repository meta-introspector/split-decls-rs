macro_rules! impl_const_default {
    () => {
        # [cfg (feature = "const-default")] mod impl_const_default ;
    };
}

impl_const_default!();