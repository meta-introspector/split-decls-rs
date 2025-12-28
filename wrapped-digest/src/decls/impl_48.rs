macro_rules! deps {
    () => {
        FixedOutputReset!();
        FixedOutput!();
        Update!();
        HashMarker!();
        Digest!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < D : FixedOutput + Default + Update + HashMarker > Digest for D { # [inline] fn new () -> Self { Self :: default () } # [inline] fn new_with_prefix (data : impl AsRef < [u8] >) -> Self where Self : Default + Sized , { let mut h = Self :: default () ; h . update (data . as_ref ()) ; h } # [inline] fn update (& mut self , data : impl AsRef < [u8] >) { Update :: update (self , data . as_ref ()) ; } # [inline] fn chain_update (mut self , data : impl AsRef < [u8] >) -> Self { Update :: update (& mut self , data . as_ref ()) ; self } # [inline] fn finalize (self) -> Output < Self > { FixedOutput :: finalize_fixed (self) } # [inline] fn finalize_into (self , out : & mut Output < Self >) { FixedOutput :: finalize_into (self , out) ; } # [inline] fn finalize_reset (& mut self) -> Output < Self > where Self : FixedOutputReset , { FixedOutputReset :: finalize_fixed_reset (self) } # [inline] fn finalize_into_reset (& mut self , out : & mut Output < Self >) where Self : FixedOutputReset , { FixedOutputReset :: finalize_into_reset (self , out) ; } # [inline] fn reset (& mut self) where Self : Reset , { Reset :: reset (self) } # [inline] fn output_size () -> usize { Self :: OutputSize :: to_usize () } # [inline] fn digest (data : impl AsRef < [u8] >) -> Output < Self > { let mut hasher = Self :: default () ; hasher . update (data . as_ref ()) ; hasher . finalize () } }
    };
}

impl_48!();