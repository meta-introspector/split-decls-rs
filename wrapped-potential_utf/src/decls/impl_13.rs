macro_rules! deps {
    () => {
        PotentialCodePoint!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl TryFrom < u32 > for PotentialCodePoint { type Error = () ; fn try_from (x : u32) -> Result < Self , () > { let [u0 , u1 , u2 , u3] = x . to_le_bytes () ; if u3 != 0 { return Err (()) ; } Ok (Self ([u0 , u1 , u2])) } }
    };
}

impl_13!();