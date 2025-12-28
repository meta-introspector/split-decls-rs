macro_rules! deps {
    () => {
        Hole!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        impl < 'a , T > Hole < 'a , T > { # [doc = " Create a new Hole at index `pos`."] # [doc = ""] # [doc = " Unsafe because pos must be within the data slice."] # [inline] unsafe fn new (data : & 'a mut [T] , pos : usize) -> Self { debug_assert ! (pos < data . len ()) ; let elt = ptr :: read (data . get_unchecked (pos)) ; Hole { data , elt : ManuallyDrop :: new (elt) , pos , } } # [inline] fn pos (& self) -> usize { self . pos } # [doc = " Returns a reference to the element removed."] # [inline] fn element (& self) -> & T { & self . elt } # [doc = " Returns a reference to the element at `index`."] # [doc = ""] # [doc = " Unsafe because index must be within the data slice and not equal to pos."] # [inline] unsafe fn get (& self , index : usize) -> & T { debug_assert ! (index != self . pos) ; debug_assert ! (index < self . data . len ()) ; self . data . get_unchecked (index) } # [doc = " Move hole to new location"] # [doc = ""] # [doc = " Unsafe because index must be within the data slice and not equal to pos."] # [inline] unsafe fn move_to (& mut self , index : usize) { debug_assert ! (index != self . pos) ; debug_assert ! (index < self . data . len ()) ; let ptr = self . data . as_mut_ptr () ; let index_ptr : * const _ = ptr . add (index) ; let hole_ptr = ptr . add (self . pos) ; ptr :: copy_nonoverlapping (index_ptr , hole_ptr , 1) ; self . pos = index ; } }
    };
}

impl_372!()