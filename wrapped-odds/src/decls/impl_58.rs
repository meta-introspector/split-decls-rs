macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < T > IndexMut < usize > for RevSlice < T > { fn index_mut (& mut self , i : usize) -> & mut T { let len = self . len () ; if let Some (x) = self . get_mut (i) { return x ; } else { panic ! ("Index {} is out of bounds for RevSlice of length {}" , i , len) ; } } }
    };
}

impl_58!();