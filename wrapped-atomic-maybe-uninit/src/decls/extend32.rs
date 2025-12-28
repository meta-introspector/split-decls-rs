macro_rules! deps {
    () => {
        Extended!();
    };
}

macro_rules! extend32 {
    () => {
        deps!();
        # [cfg (not (target_pointer_width = "16"))] # [allow (dead_code)] pub (crate) mod extend32 { macro_rules ! extend { ($ ($ ty : ident) ,* => $ out : ident) => { $ (pub (crate) mod $ ty { use core :: mem :: { self , MaybeUninit } ; use super :: super :: Extended ; const LEN : usize = (mem :: size_of ::<$ out > () - mem :: size_of ::<$ ty > ()) / mem :: size_of ::<$ ty > () ; # [allow (clippy :: cast_sign_loss)] const _ : () = assert ! (unsafe { zero (MaybeUninit :: new (! 0)) . assume_init () == ! (0 as $ ty) as $ out }) ; # [doc = " Zero-extends the given integer to `MaybeUninit<u32>` if it is smaller than 32-bit,"] # [doc = " otherwise, return the given value as-is."] # [inline (always)] pub (crate) const fn zero (v : MaybeUninit <$ ty >) -> MaybeUninit <$ out > { const PAD : [MaybeUninit <$ ty >; LEN] = [MaybeUninit :: new (0) ; LEN] ; unsafe { mem :: transmute (Extended ::<$ ty , LEN > { v , pad : PAD }) } } # [doc = " Uninit-extends the given integer to `MaybeUninit<u32>` if it is smaller than 32-bit,"] # [doc = " otherwise, return the given value as-is."] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] # [inline (always)] pub (crate) const fn uninit (v : MaybeUninit <$ ty >) -> MaybeUninit <$ out > { const PAD : [MaybeUninit <$ ty >; LEN] = [MaybeUninit :: uninit () ; LEN] ; unsafe { mem :: transmute (Extended ::<$ ty , LEN > { v , pad : PAD }) } } # [doc = " Inverse of extend."] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] # [inline (always)] pub (crate) const fn extract (v : MaybeUninit <$ out >) -> MaybeUninit <$ ty > { unsafe { mem :: transmute ::< MaybeUninit <$ out >, Extended ::<$ ty , LEN >> (v) . v } } # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] # [inline (always)] pub (crate) const fn identity (v : MaybeUninit <$ ty >) -> MaybeUninit <$ ty > { v } }) * } ; ($ ($ ty : ident) ,*) => { $ (pub (crate) mod $ ty { use core :: mem :: MaybeUninit ; # [inline (always)] pub (crate) const fn identity (v : MaybeUninit <$ ty >) -> MaybeUninit <$ ty > { v } # [allow (unused_imports)] pub (crate) use self :: identity as zero ; # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] # [allow (unused_imports)] pub (crate) use self :: { identity as uninit , identity as extract } ; }) * } ; } extend ! (u8 , u16 => u32) ; extend ! (u32 , u64) ; }
    };
}

extend32!();