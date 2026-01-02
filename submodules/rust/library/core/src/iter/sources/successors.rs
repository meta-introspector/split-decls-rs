mkuse!{use crate :: fmt ;}
mkuse!{use crate :: iter :: FusedIterator ;}

macro_rules! successors_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function successors in module {}", module_path!());
    };
}

mkfn!{
    successors_introspect!();
    # [doc = " Creates an iterator which, starting from an initial item,"] # [doc = " computes each successive item from the preceding one."] # [doc = ""] # [doc = " This iterator stores an optional item (`Option<T>`) and a successor closure (`impl FnMut(&T) -> Option<T>`)."] # [doc = " Its `next` method returns the stored optional item and"] # [doc = " if it is `Some(val)` calls the stored closure on `&val` to compute and store its successor."] # [doc = " The iterator will apply the closure successively to the stored option's value until the option is `None`."] # [doc = " This also means that once the stored option is `None` it will remain `None`,"] # [doc = " as the closure will not be called again, so the created iterator is a [`FusedIterator`]."] # [doc = " The iterator's items will be the initial item and all of its successors as calculated by the successor closure."] # [doc = ""] # [doc = " ```"] # [doc = " use std::iter::successors;"] # [doc = ""] # [doc = " let powers_of_10 = successors(Some(1_u16), |n| n.checked_mul(10));"] # [doc = " assert_eq!(powers_of_10.collect::<Vec<_>>(), &[1, 10, 100, 1_000, 10_000]);"] # [doc = " ```"] # [stable (feature = "iter_successors" , since = "1.34.0")] pub fn successors < T , F > (first : Option < T > , succ : F) -> Successors < T , F > where F : FnMut (& T) -> Option < T > , { Successors { next : first , succ } }
}
mkitem!{mkstruct!{# [doc = " An iterator which, starting from an initial item,"] # [doc = " computes each successive item from the preceding one."] # [doc = ""] # [doc = " This `struct` is created by the [`iter::successors()`] function."] # [doc = " See its documentation for more."] # [doc = ""] # [doc = " [`iter::successors()`]: successors"] # [derive (Clone)] # [stable (feature = "iter_successors" , since = "1.34.0")] pub struct Successors < T , F > { next : Option < T > , succ : F , }}}
mkitem!{mkimpl!{# [stable (feature = "iter_successors" , since = "1.34.0")] impl < T , F > Iterator for Successors < T , F > where F : FnMut (& T) -> Option < T > , { type Item = T ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let item = self . next . take () ? ; self . next = (self . succ) (& item) ; Some (item) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { if self . next . is_some () { (1 , None) } else { (0 , Some (0)) } } }}}
mkitem!{mkimpl!{# [stable (feature = "iter_successors" , since = "1.34.0")] impl < T , F > FusedIterator for Successors < T , F > where F : FnMut (& T) -> Option < T > { }}}
mkitem!{mkimpl!{# [stable (feature = "iter_successors" , since = "1.34.0")] impl < T : fmt :: Debug , F > fmt :: Debug for Successors < T , F > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Successors") . field ("next" , & self . next) . finish () } }}}