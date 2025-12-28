macro_rules! deps {
    () => {
        Extended!();
    };
}

macro_rules! zero_extend64 {
    () => {
        deps!();
        # [cfg (target_pointer_width = "32")] # [allow (dead_code)] pub (crate) mod zero_extend64 { use core :: mem :: { self , MaybeUninit } ; use super :: Extended ; const _ : () = assert ! (unsafe { ptr (super :: ptr :: without_provenance_mut (! 0)) . assume_init () == ! 0_u32 as u64 }) ; # [doc = " Zero-extends the given 32-bit pointer to `MaybeUninit<u64>`."] # [doc = " This is used for 64-bit architecture's 32-bit ABI (e.g., AArch64 ILP32 ABI)."] # [doc = " See ptr_reg! macro in src/gen/utils.rs for details."] # [inline] pub (crate) const fn ptr (v : * mut ()) -> MaybeUninit < u64 > { const PAD : [MaybeUninit < * mut () > ; 1] = [MaybeUninit :: new (core :: ptr :: null_mut ()) ; 1] ; unsafe { mem :: transmute (Extended :: < * mut () , 1 > { v : MaybeUninit :: new (v) , pad : PAD }) } } }
    };
}

zero_extend64!()