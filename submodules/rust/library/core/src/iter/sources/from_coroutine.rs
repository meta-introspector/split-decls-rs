mkuse!{use crate :: fmt ;}
mkuse!{use crate :: ops :: { Coroutine , CoroutineState } ;}
mkuse!{use crate :: pin :: Pin ;}

macro_rules! from_coroutine_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_coroutine in module {}", module_path!());
    };
}

mkfn!{
    from_coroutine_introspect!();
    # [doc = " Creates a new iterator where each iteration calls the provided coroutine."] # [doc = ""] # [doc = " Similar to [`iter::from_fn`]."] # [doc = ""] # [doc = " [`iter::from_fn`]: crate::iter::from_fn"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(coroutines)]"] # [doc = " #![feature(iter_from_coroutine)]"] # [doc = ""] # [doc = " let it = std::iter::from_coroutine(#[coroutine] || {"] # [doc = "     yield 1;"] # [doc = "     yield 2;"] # [doc = "     yield 3;"] # [doc = " });"] # [doc = " let v: Vec<_> = it.collect();"] # [doc = " assert_eq!(v, [1, 2, 3]);"] # [doc = " ```"] # [inline] # [unstable (feature = "iter_from_coroutine" , issue = "43122" , reason = "coroutines are unstable")] pub fn from_coroutine < G : Coroutine < Return = () > + Unpin > (coroutine : G) -> FromCoroutine < G > { FromCoroutine (coroutine) }
}
mkitem!{mkstruct!{# [doc = " An iterator over the values yielded by an underlying coroutine."] # [doc = ""] # [doc = " This `struct` is created by the [`iter::from_coroutine()`] function. See its documentation for"] # [doc = " more."] # [doc = ""] # [doc = " [`iter::from_coroutine()`]: from_coroutine"] # [unstable (feature = "iter_from_coroutine" , issue = "43122" , reason = "coroutines are unstable")] # [derive (Clone)] pub struct FromCoroutine < G > (G) ;}}
mkitem!{mkimpl!{# [unstable (feature = "iter_from_coroutine" , issue = "43122" , reason = "coroutines are unstable")] impl < G : Coroutine < Return = () > + Unpin > Iterator for FromCoroutine < G > { type Item = G :: Yield ; fn next (& mut self) -> Option < Self :: Item > { match Pin :: new (& mut self . 0) . resume (()) { CoroutineState :: Yielded (n) => Some (n) , CoroutineState :: Complete (()) => None , } } }}}
mkitem!{mkimpl!{# [unstable (feature = "iter_from_coroutine" , issue = "43122" , reason = "coroutines are unstable")] impl < G > fmt :: Debug for FromCoroutine < G > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FromCoroutine") . finish () } }}}