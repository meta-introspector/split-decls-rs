macro_rules! deps {
    () => {
        ArrayVec!();
        ArrayLike!();
        Result!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < A : ArrayLike > ArrayVec < A > { pub fn new () -> Self { Self { storage : A :: new_storage () , len : 0 , } } pub fn clear (& mut self) { let ptr : * mut [A :: Item] = & mut * * self ; self . len = 0 ; unsafe { ptr :: drop_in_place (ptr) } ; } pub fn try_push (& mut self , value : A :: Item) -> Result < () , CapacityFull > { let mut storage = A :: as_mut_slice (& mut self . storage) ; if self . len >= storage . len () { A :: grow (& mut self . storage , 1) ? ; storage = A :: as_mut_slice (& mut self . storage) ; } storage [self . len] = MaybeUninit :: new (value) ; self . len += 1 ; Ok (()) } pub fn try_insert (& mut self , index : usize , element : A :: Item) -> Result < () , CapacityFull > { assert ! (index <= self . len) ; let mut storage = A :: as_mut_slice (& mut self . storage) ; if self . len >= storage . len () { A :: grow (& mut self . storage , 1) ? ; storage = A :: as_mut_slice (& mut self . storage) ; } unsafe { let p = storage . as_mut_ptr () . add (index) ; core :: ptr :: copy (p as * const _ , p . add (1) , self . len - index) ; } storage [index] = MaybeUninit :: new (element) ; self . len += 1 ; Ok (()) } pub fn pop (& mut self) -> Option < A :: Item > { if self . len == 0 { None } else { self . len -= 1 ; Some (unsafe { A :: as_slice (& self . storage) [self . len] . as_ptr () . read () }) } } pub fn swap_remove (& mut self , index : usize) -> A :: Item { assert ! (self . len > 0) ; A :: as_mut_slice (& mut self . storage) . swap (index , self . len - 1) ; self . pop () . unwrap () } }
    };
}

impl_132!()