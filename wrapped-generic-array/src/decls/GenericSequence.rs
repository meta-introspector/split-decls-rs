macro_rules! deps {
    () => {
        MappedSequence!();
        ArrayLength!();
        GenericArray!();
        IntrusiveArrayConsumer!();
        MappedGenericSequence!();
    };
}

macro_rules! GenericSequence {
    () => {
        deps!();
        # [doc = " Defines some sequence with an associated length and iteration capabilities."] # [doc = ""] # [doc = " This is useful for passing N-length generic arrays as generics."] # [doc = ""] # [doc = " # Safety"] # [doc = " Care must be taken when implementing such that methods are safe."] # [doc = ""] # [doc = " Lengths must match, and element drop on panic must be handled."] pub unsafe trait GenericSequence < T > : Sized + IntoIterator { # [doc = " `GenericArray` associated length"] type Length : ArrayLength ; # [doc = " Owned sequence type used in conjunction with reference implementations of `GenericSequence`"] type Sequence : GenericSequence < T , Length = Self :: Length > + FromIterator < T > ; # [doc = " Initializes a new sequence instance using the given function."] # [doc = ""] # [doc = " If the generator function panics while initializing the sequence,"] # [doc = " any already initialized elements will be dropped."] # [doc = ""] # [doc = " See also [`FallibleGenericSequence::try_generate`]."] fn generate < F > (f : F) -> Self :: Sequence where F : FnMut (usize) -> T ; # [doc = " Initializes a new sequence instance by repeating the given value."] # [doc = ""] # [doc = " This will only clone the value `Length - 1` times, taking ownership for the last element."] # [inline (always)] fn repeat (value : T) -> Self :: Sequence where T : Clone , { let mut value = Some (value) ; Self :: generate (move | i | unsafe { if i + 1 == Self :: Length :: USIZE { value . take () . unwrap_unchecked () } else if let Some (ref v) = value { v . clone () } else { core :: hint :: unreachable_unchecked () } }) } # [doc = " Treats `self` as the right-hand operand in a zip operation"] # [doc = ""] # [doc = " This is optimized for stack-allocated `GenericArray`s"] # [cfg_attr (not (feature = "internals") , doc (hidden))] # [inline (always)] fn inverted_zip < B , U , F > (self , lhs : GenericArray < B , Self :: Length > , mut f : F ,) -> MappedSequence < GenericArray < B , Self :: Length > , B , U > where GenericArray < B , Self :: Length > : GenericSequence < B , Length = Self :: Length > + MappedGenericSequence < B , U > , Self : MappedGenericSequence < T , U > , F : FnMut (B , Self :: Item) -> U , { unsafe { let mut left = ManuallyDrop :: new (lhs) ; let mut left = IntrusiveArrayConsumer :: new (& mut left) ; let (left_array_iter , left_position) = left . iter_position () ; FromIterator :: from_iter (left_array_iter . zip (self) . map (| (l , right_value) | { let left_value = ptr :: read (l) ; * left_position += 1 ; f (left_value , right_value) })) } } # [doc = " Treats `self` as the right-hand operand in a zip operation"] # [cfg_attr (not (feature = "internals") , doc (hidden))] # [inline (always)] fn inverted_zip2 < B , Lhs , U , F > (self , lhs : Lhs , mut f : F) -> MappedSequence < Lhs , B , U > where Lhs : GenericSequence < B , Length = Self :: Length > + MappedGenericSequence < B , U > , Self : MappedGenericSequence < T , U > , F : FnMut (Lhs :: Item , Self :: Item) -> U , { FromIterator :: from_iter (lhs . into_iter () . zip (self) . map (| (l , r) | f (l , r))) } }
    };
}

GenericSequence!()