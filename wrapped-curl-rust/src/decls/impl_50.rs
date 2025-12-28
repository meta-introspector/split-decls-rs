macro_rules! deps {
    () => {
        Callbacks!();
        EasyData!();
        Transfer!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl EasyData { # [doc = " An unsafe function to get the appropriate callback field."] # [doc = ""] # [doc = " We can have callbacks configured from one of two different sources."] # [doc = " We could either have a callback from the `borrowed` field, callbacks on"] # [doc = " an ephemeral `Transfer`, or the `owned` field which are `'static`"] # [doc = " callbacks that live for the lifetime of this `EasyData`."] # [doc = ""] # [doc = " The first set of callbacks are unsafe to access because they're actually"] # [doc = " owned elsewhere and we're just aliasing. Additionally they don't"] # [doc = " technically live long enough for us to access them, so they're hidden"] # [doc = " behind unsafe pointers and casts."] # [doc = ""] # [doc = " This function returns `&'a mut T` but that's actually somewhat of a lie."] # [doc = " The value should **not be stored to** nor should it be used for the full"] # [doc = " lifetime of `'a`, but rather immediately in the local scope."] # [doc = ""] # [doc = " Basically this is just intended to acquire a callback, invoke it, and"] # [doc = " then stop. Nothing else. Super unsafe."] unsafe fn callback < 'a , T , F > (& 'a mut self , f : F) -> Option < & 'a mut T > where F : for < 'b > Fn (& 'b mut Callbacks < 'static >) -> & 'b mut Option < T > , { let ptr = self . borrowed . get () ; if ! ptr . is_null () { let val = f (& mut * ptr) ; if val . is_some () { return val . as_mut () ; } } f (& mut self . owned) . as_mut () } }
    };
}

impl_50!();