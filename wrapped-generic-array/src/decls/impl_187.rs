macro_rules! deps {
    () => {
        GenericArray!();
        IntrusiveArrayConsumer!();
        MappedGenericSequence!();
        IntrusiveArrayBuilder!();
        GenericSequence!();
        MappedSequence!();
        ArrayLength!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        unsafe impl < T , N : ArrayLength > GenericSequence < T > for GenericArray < T , N > where Self : IntoIterator < Item = T > , { type Length = N ; type Sequence = Self ; # [inline (always)] fn generate < F > (mut f : F) -> GenericArray < T , N > where F : FnMut (usize) -> T , { unsafe { let mut array = MaybeUninit :: < GenericArray < T , N > > :: uninit () ; let mut builder = IntrusiveArrayBuilder :: new_alt (& mut array) ; let (builder_iter , position) = builder . iter_position () ; builder_iter . enumerate () . for_each (| (i , dst) | { dst . write (f (i)) ; * position += 1 ; }) ; builder . finish_and_assume_init () } } # [inline (always)] fn inverted_zip < B , U , F > (self , lhs : GenericArray < B , Self :: Length > , mut f : F ,) -> MappedSequence < GenericArray < B , Self :: Length > , B , U > where GenericArray < B , Self :: Length > : GenericSequence < B , Length = Self :: Length > + MappedGenericSequence < B , U > , Self : MappedGenericSequence < T , U > , F : FnMut (B , Self :: Item) -> U , { unsafe { let mut left = ManuallyDrop :: new (lhs) ; let mut right = ManuallyDrop :: new (self) ; if mem :: needs_drop :: < T > () || mem :: needs_drop :: < B > () { let mut left = IntrusiveArrayConsumer :: new (& mut left) ; let mut right = IntrusiveArrayConsumer :: new (& mut right) ; let (left_array_iter , left_position) = left . iter_position () ; let (right_array_iter , right_position) = right . iter_position () ; FromIterator :: from_iter (left_array_iter . zip (right_array_iter) . map (| (l , r) | { let left_value = ptr :: read (l) ; let right_value = ptr :: read (r) ; * left_position += 1 ; * right_position = * left_position ; f (left_value , right_value) })) } else { FromIterator :: from_iter (left . iter () . zip (right . iter ()) . map (| (l , r) | { f (ptr :: read (l) , ptr :: read (r)) })) } } } # [inline (always)] fn inverted_zip2 < B , Lhs , U , F > (self , lhs : Lhs , mut f : F) -> MappedSequence < Lhs , B , U > where Lhs : GenericSequence < B , Length = Self :: Length > + MappedGenericSequence < B , U > , Self : MappedGenericSequence < T , U > , F : FnMut (Lhs :: Item , Self :: Item) -> U , { unsafe { if mem :: needs_drop :: < T > () { let mut right = ManuallyDrop :: new (self) ; let mut right = IntrusiveArrayConsumer :: new (& mut right) ; let (right_array_iter , right_position) = right . iter_position () ; FromIterator :: from_iter (right_array_iter . zip (lhs) . map (| (r , left_value) | { let right_value = ptr :: read (r) ; * right_position += 1 ; f (left_value , right_value) })) } else { let right = ManuallyDrop :: new (self) ; FromIterator :: from_iter (right . iter () . zip (lhs) . map (| (r , left_value) | { f (left_value , ptr :: read (r)) })) } } } }
    };
}

impl_187!()