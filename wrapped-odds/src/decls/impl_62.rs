macro_rules! deps {
    () => {
        RevSlice!();
        IndexRange!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T , R > IndexMut < R > for RevSlice < T > where R : IndexRange , { fn index_mut (& mut self , index : R) -> & mut RevSlice < T > { let start = index . start () . unwrap_or (0) ; let end = index . end () . unwrap_or (self . len ()) ; assert ! (start <= end && end <= self . len ()) ; let end_r = self . len () - start ; let start_r = self . len () - end ; unsafe { < & mut RevSlice < _ > > :: from (get_unchecked_mut (& mut self . 0 , start_r .. end_r)) } } }
    };
}

impl_62!();