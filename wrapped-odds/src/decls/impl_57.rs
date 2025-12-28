macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < T > Index < usize > for RevSlice < T > { type Output = T ; fn index (& self , i : usize) -> & T { if let Some (x) = self . get (i) { x } else { panic ! ("Index {} is out of bounds for RevSlice of length {}" , i , self . len ()) ; } } }
    };
}

impl_57!()