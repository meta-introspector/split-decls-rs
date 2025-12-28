macro_rules! deps {
    () => {
        ExactlyOneError!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl < I > Iterator for ExactlyOneError < I > where I : Iterator , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { match self . first_two . take () { Some (Either :: Left ([first , second])) => { self . first_two = Some (Either :: Right (second)) ; Some (first) } Some (Either :: Right (second)) => Some (second) , None => self . inner . next () , } } fn size_hint (& self) -> (usize , Option < usize >) { size_hint :: add_scalar (self . inner . size_hint () , self . additional_len ()) } fn count (self) -> usize where Self : Sized , { self . additional_len () + self . inner . count () } fn fold < B , F > (self , mut init : B , mut f : F) -> B where F : FnMut (B , Self :: Item) -> B , { match self . first_two { Some (Either :: Left ([first , second])) => { init = f (init , first) ; init = f (init , second) ; } Some (Either :: Right (second)) => init = f (init , second) , None => { } } self . inner . fold (init , f) } }
    };
}

impl_219!()