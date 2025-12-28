macro_rules! ShrVartime {
    () => {
        # [doc = " Right shifts, variable time in `shift`."] pub trait ShrVartime : Sized { # [doc = " Computes `self >> shift`."] # [doc = ""] # [doc = " Returns `None` if `shift >= self.bits_precision()`."] fn overflowing_shr_vartime (& self , shift : u32) -> CtOption < Self > ; # [doc = " Computes `self >> shift` in a panic-free manner, masking off bits of `shift`"] # [doc = " which would cause the shift to exceed the type's width."] fn wrapping_shr_vartime (& self , shift : u32) -> Self ; }
    };
}

ShrVartime!()