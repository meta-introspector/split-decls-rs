macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        # [doc = " Extend the `ArrayVec` with an iterator."] # [doc = ""] # [doc = " ***Panics*** if extending the vector exceeds its capacity."] impl < T , const CAP : usize > Extend < T > for ArrayVec < T , CAP > { # [doc = " Extend the `ArrayVec` with an iterator."] # [doc = ""] # [doc = " ***Panics*** if extending the vector exceeds its capacity."] # [track_caller] fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { unsafe { self . extend_from_iter :: < _ , true > (iter) } } }
    };
}

impl_67!()