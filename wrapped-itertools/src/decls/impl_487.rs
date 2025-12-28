macro_rules! deps {
    () => {
        Iterate!();
    };
}

macro_rules! impl_487 {
    () => {
        deps!();
        impl < St , F > Iterator for Iterate < St , F > where F : FnMut (& St) -> St , { type Item = St ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let next_state = (self . f) (& self . state) ; Some (mem :: replace (& mut self . state , next_state)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }
    };
}

impl_487!();