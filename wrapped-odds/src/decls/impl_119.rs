macro_rules! deps {
    () => {
        StrideMut!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < 'a , A > IndexMut < usize > for StrideMut < 'a , A > { # [doc = " Return a mutable reference to the element at a given index."] # [doc = ""] # [doc = " **Panics** if the index is out of bounds."] fn index_mut < 'b > (& 'b mut self , i : usize) -> & 'b mut A { assert ! (i < self . len ()) ; unsafe { let ptr = self . begin . offset (self . offset + self . stride * (i as isize)) ; & mut * ptr } } }
    };
}

impl_119!()