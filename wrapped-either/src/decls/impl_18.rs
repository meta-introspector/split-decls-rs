macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        # [doc = " `Either<L, R>` is an iterator if both `L` and `R` are iterators."] impl < L , R > Iterator for Either < L , R > where L : Iterator , R : Iterator < Item = L :: Item > , { type Item = L :: Item ; fn next (& mut self) -> Option < Self :: Item > { for_both ! (self , inner => inner . next ()) } fn size_hint (& self) -> (usize , Option < usize >) { for_both ! (self , inner => inner . size_hint ()) } fn fold < Acc , G > (self , init : Acc , f : G) -> Acc where G : FnMut (Acc , Self :: Item) -> Acc , { for_both ! (self , inner => inner . fold (init , f)) } fn for_each < F > (self , f : F) where F : FnMut (Self :: Item) , { for_both ! (self , inner => inner . for_each (f)) } fn count (self) -> usize { for_both ! (self , inner => inner . count ()) } fn last (self) -> Option < Self :: Item > { for_both ! (self , inner => inner . last ()) } fn nth (& mut self , n : usize) -> Option < Self :: Item > { for_both ! (self , inner => inner . nth (n)) } fn collect < B > (self) -> B where B : iter :: FromIterator < Self :: Item > , { for_both ! (self , inner => inner . collect ()) } fn partition < B , F > (self , f : F) -> (B , B) where B : Default + Extend < Self :: Item > , F : FnMut (& Self :: Item) -> bool , { for_both ! (self , inner => inner . partition (f)) } fn all < F > (& mut self , f : F) -> bool where F : FnMut (Self :: Item) -> bool , { for_both ! (self , inner => inner . all (f)) } fn any < F > (& mut self , f : F) -> bool where F : FnMut (Self :: Item) -> bool , { for_both ! (self , inner => inner . any (f)) } fn find < P > (& mut self , predicate : P) -> Option < Self :: Item > where P : FnMut (& Self :: Item) -> bool , { for_both ! (self , inner => inner . find (predicate)) } fn find_map < B , F > (& mut self , f : F) -> Option < B > where F : FnMut (Self :: Item) -> Option < B > , { for_both ! (self , inner => inner . find_map (f)) } fn position < P > (& mut self , predicate : P) -> Option < usize > where P : FnMut (Self :: Item) -> bool , { for_both ! (self , inner => inner . position (predicate)) } }
    };
}

impl_18!()