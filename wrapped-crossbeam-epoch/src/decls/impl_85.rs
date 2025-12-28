macro_rules! deps {
    () => {
        Data!();
        Deferred!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl Deferred { pub (crate) const NO_OP : Self = { fn no_op_call (_raw : * mut u8) { } Self { call : no_op_call , data : MaybeUninit :: uninit () , _marker : PhantomData , } } ; # [doc = " Constructs a new `Deferred` from a `FnOnce()`."] pub (crate) fn new < F : FnOnce () > (f : F) -> Self { let size = mem :: size_of :: < F > () ; let align = mem :: align_of :: < F > () ; unsafe { if size <= mem :: size_of :: < Data > () && align <= mem :: align_of :: < Data > () { let mut data = MaybeUninit :: < Data > :: uninit () ; ptr :: write (data . as_mut_ptr () . cast :: < F > () , f) ; unsafe fn call < F : FnOnce () > (raw : * mut u8) { let f : F = unsafe { ptr :: read (raw . cast :: < F > ()) } ; f () ; } Self { call : call :: < F > , data , _marker : PhantomData , } } else { let b : Box < F > = Box :: new (f) ; let mut data = MaybeUninit :: < Data > :: uninit () ; ptr :: write (data . as_mut_ptr () . cast :: < Box < F > > () , b) ; unsafe fn call < F : FnOnce () > (raw : * mut u8) { let b : Box < F > = unsafe { ptr :: read (raw . cast :: < Box < F > > ()) } ; (* b) () ; } Self { call : call :: < F > , data , _marker : PhantomData , } } } } # [doc = " Calls the function."] # [inline] pub (crate) fn call (mut self) { let call = self . call ; unsafe { call (self . data . as_mut_ptr () . cast :: < u8 > ()) } ; } }
    };
}

impl_85!();