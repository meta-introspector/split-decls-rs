macro_rules! deps {
    () => {
        RevSlice!();
        IndexRange!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < T , R > Index < R > for RevSlice < T > where R : IndexRange , { type Output = RevSlice < T > ; fn index (& self , index : R) -> & RevSlice < T > { let start = index . start () . unwrap_or (0) ; let end = index . end () . unwrap_or (self . len ()) ; assert ! (start <= end && end <= self . len ()) ; let end_r = self . len () - start ; let start_r = self . len () - end ; unsafe { < & RevSlice < _ > > :: from (get_unchecked (& self . 0 , start_r .. end_r)) } } }
    };
}

impl_61!();