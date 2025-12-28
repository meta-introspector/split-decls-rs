macro_rules! deps {
    () => {
        StaticValue!();
        Lazy!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl < T : 'static > Lazy < T > { # [doc = " Mock implementation of `lazy_static::Lazy::get`."] pub fn get (& 'static self) -> & 'static T { match unsafe { self . try_get () } { Some (v) => v , None => { let sv = crate :: rt :: lazy_static :: StaticValue :: new ((self . init) ()) ; if let Some (v) = unsafe { self . try_get () } { return v ; } rt :: execution (| execution | { let sv = execution . lazy_statics . init_static (self , sv) ; sv . sync . sync_store (& mut execution . threads , Ordering :: AcqRel) ; }) ; unsafe { self . try_get () } . expect ("bug") } } } unsafe fn try_get (& 'static self) -> Option < & 'static T > { unsafe fn transmute_lt < 'a , 'b , T > (t : & 'a T) -> & 'b T { std :: mem :: transmute :: < & 'a T , & 'b T > (t) } let sv = rt :: execution (| execution | { let sv = execution . lazy_statics . get_static (self) ? ; sv . sync . sync_load (& mut execution . threads , Ordering :: Acquire) ; Some (transmute_lt (sv)) }) ? ; Some (sv . get :: < T > ()) } }
    };
}

impl_226!();