macro_rules! deps {
    () => {
        VecType!();
        Limb!();
        StackVec!();
    };
}

macro_rules! from_u64 {
    () => {
        deps!();
        # [doc = " Create StackVec from u64 value."] # [inline (always)] # [allow (clippy :: branches_sharing_code)] pub fn from_u64 (x : u64) -> VecType { let mut vec = VecType :: new () ; debug_assert ! (vec . capacity () >= 2) ; if LIMB_BITS == 32 { vec . try_push (x as Limb) . unwrap () ; vec . try_push ((x >> 32) as Limb) . unwrap () ; } else { vec . try_push (x as Limb) . unwrap () ; } vec . normalize () ; vec }
    };
}

from_u64!()