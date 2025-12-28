macro_rules! deps {
    () => {
        CountItem!();
        CoalesceBy!();
        CoalescePredicate!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < I , F , C > Iterator for CoalesceBy < I , F , C > where I : Iterator , F : CoalescePredicate < I :: Item , C :: CItem > , C : CountItem < I :: Item > , { type Item = C :: CItem ; fn next (& mut self) -> Option < Self :: Item > { let Self { iter , last , f } = self ; let init = match last { Some (elt) => elt . take () , None => { * last = Some (None) ; iter . next () . map (C :: new) } } ? ; Some (iter . try_fold (init , | accum , next | match f . coalesce_pair (accum , next) { Ok (joined) => Ok (joined) , Err ((last_ , next_)) => { * last = Some (Some (next_)) ; Err (last_) } }) . unwrap_or_else (| x | x) ,) } fn size_hint (& self) -> (usize , Option < usize >) { let (low , hi) = size_hint :: add_scalar (self . iter . size_hint () , matches ! (self . last , Some (Some (_))) as usize ,) ; ((low > 0) as usize , hi) } fn fold < Acc , FnAcc > (self , acc : Acc , mut fn_acc : FnAcc) -> Acc where FnAcc : FnMut (Acc , Self :: Item) -> Acc , { let Self { mut iter , last , mut f , } = self ; if let Some (last) = last . unwrap_or_else (| | iter . next () . map (C :: new)) { let (last , acc) = iter . fold ((last , acc) , | (last , acc) , elt | { match f . coalesce_pair (last , elt) { Ok (joined) => (joined , acc) , Err ((last_ , next_)) => (next_ , fn_acc (acc , last_)) , } }) ; fn_acc (acc , last) } else { acc } } }
    };
}

impl_15!()