macro_rules! deps {
    () => {
        Fn1!();
        FnMut1!();
        FnOnce1!();
    };
}

macro_rules! trivial_fn_impls {
    () => {
        deps!();
        macro_rules ! trivial_fn_impls { ($ name : ident <$ ($ arg : ident) ,*> $ t : ty = $ debug : literal) => { impl <$ ($ arg) ,*> Copy for $ t { } impl <$ ($ arg) ,*> Clone for $ t { fn clone (& self) -> Self { * self } } impl <$ ($ arg) ,*> Debug for $ t { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { f . write_str ($ debug) } } impl <$ ($ arg ,) * A > FnMut1 < A > for $ t where Self : FnOnce1 < A > { fn call_mut (& mut self , arg : A) -> Self :: Output { self . call_once (arg) } } impl <$ ($ arg ,) * A > Fn1 < A > for $ t where Self : FnOnce1 < A > { fn call (& self , arg : A) -> Self :: Output { self . call_once (arg) } } pub (crate) fn $ name <$ ($ arg) ,*> () -> $ t { Default :: default () } } }
    };
}

trivial_fn_impls!();