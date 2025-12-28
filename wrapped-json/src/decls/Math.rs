macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! Math {
    () => {
        deps!();
        # [doc = " Traits for shared operations for big integers."] # [doc = ""] # [doc = " None of these are implemented using normal traits, since these"] # [doc = " are very expensive operations, and we want to deliberately"] # [doc = " and explicitly use these functions."] pub (crate) trait Math : Clone + Sized + Default { # [doc = " Get access to the underlying data"] fn data (& self) -> & Vec < Limb > ; # [doc = " Get access to the underlying data"] fn data_mut (& mut self) -> & mut Vec < Limb > ; # [doc = " Compare self to y."] # [inline] fn compare (& self , y : & Self) -> cmp :: Ordering { large :: compare (self . data () , y . data ()) } # [doc = " Get the high 64-bits from the bigint and if there are remaining bits."] # [inline] fn hi64 (& self) -> (u64 , bool) { self . data () . as_slice () . hi64 () } # [doc = " Calculate the bit-length of the big-integer."] # [doc = " Returns usize::max_value() if the value overflows,"] # [doc = " IE, if `self.data().len() > usize::max_value() / 8`."] # [inline] fn bit_length (& self) -> usize { small :: bit_length (self . data ()) } # [doc = " Create new big integer from u64."] # [inline] fn from_u64 (x : u64) -> Self { let mut v = Self :: default () ; let slc = split_u64 (x) ; v . data_mut () . extend_from_slice (& slc) ; v . normalize () ; v } # [doc = " Normalize the integer, so any leading zero values are removed."] # [inline] fn normalize (& mut self) { small :: normalize (self . data_mut ()) ; } # [doc = " AddAssign small integer."] # [inline] fn iadd_small (& mut self , y : Limb) { small :: iadd (self . data_mut () , y) ; } # [doc = " MulAssign small integer."] # [inline] fn imul_small (& mut self , y : Limb) { small :: imul (self . data_mut () , y) ; } # [doc = " Multiply by a power of 2."] # [inline] fn imul_pow2 (& mut self , n : u32) { self . ishl (n as usize) ; } # [doc = " Multiply by a power of 5."] # [inline] fn imul_pow5 (& mut self , n : u32) { small :: imul_pow5 (self . data_mut () , n) ; } # [doc = " MulAssign by a power of 10."] # [inline] fn imul_pow10 (& mut self , n : u32) { self . imul_pow5 (n) ; self . imul_pow2 (n) ; } # [doc = " Shift-left the entire buffer n bits."] # [inline] fn ishl (& mut self , n : usize) { small :: ishl (self . data_mut () , n) ; } }
    };
}

Math!()