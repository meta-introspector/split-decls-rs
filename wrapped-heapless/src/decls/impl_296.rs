macro_rules! deps {
    () => {
        LenType!();
        Vec!();
        CapacityError!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        impl < 'a , T : Clone , LenT : LenType , const N : usize > TryFrom < & 'a [T] > for Vec < T , N , LenT > { type Error = CapacityError ; fn try_from (slice : & 'a [T]) -> Result < Self , Self :: Error > { Self :: from_slice (slice) } }
    };
}

impl_296!()