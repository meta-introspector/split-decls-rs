macro_rules! deps {
    () => {
        ReadyToRunQueue!();
        Dequeue!();
        Empty!();
    };
}

macro_rules! impl_867 {
    () => {
        deps!();
        impl < Fut > Drop for ReadyToRunQueue < Fut > { fn drop (& mut self) { unsafe { loop { match self . dequeue () { Dequeue :: Empty => break , Dequeue :: Inconsistent => abort ("inconsistent in drop") , Dequeue :: Data (ptr) => drop (Arc :: from_raw (ptr)) , } } } } }
    };
}

impl_867!()