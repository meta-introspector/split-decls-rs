mkuse!{use crate :: borrow :: { Borrow , BorrowMut } ;}
mkuse!{use crate :: cmp :: Ordering ;}
mkuse!{use crate :: convert :: Infallible ;}
mkuse!{use crate :: error :: Error ;}
mkuse!{use crate :: fmt ;}
mkuse!{use crate :: hash :: { self , Hash } ;}
mkuse!{use crate :: intrinsics :: transmute_unchecked ;}
mkuse!{use crate :: iter :: { UncheckedIterator , repeat_n } ;}
mkuse!{use crate :: mem :: { self , MaybeUninit } ;}
mkuse!{use crate :: ops :: { ChangeOutputType , ControlFlow , FromResidual , Index , IndexMut , NeverShortCircuit , Residual , Try , } ;}
mkuse!{use crate :: ptr :: { null , null_mut } ;}
mkuse!{use crate :: slice :: { Iter , IterMut } ;}
mkmod!{ascii, { 
                getname!(ascii);
                getsrc!(ascii);
                getpath!(ascii);
                get_deps!(ascii);
                get_crates!(ascii);
                mkinclude!(ascii);
                 
            }}
mkmod!{drain, { 
                getname!(drain);
                getsrc!(drain);
                getpath!(drain);
                get_deps!(drain);
                get_crates!(drain);
                mkinclude!(drain);
                 
            }}
mkmod!{equality, { 
                getname!(equality);
                getsrc!(equality);
                getpath!(equality);
                get_deps!(equality);
                get_crates!(equality);
                mkinclude!(equality);
                 
            }}
mkmod!{iter, { 
                getname!(iter);
                getsrc!(iter);
                getpath!(iter);
                get_deps!(iter);
                get_crates!(iter);
                mkinclude!(iter);
                 
            }}
mkuse!{pub (crate) use drain :: drain_array_with ;}
mkuse!{# [stable (feature = "array_value_iter" , since = "1.51.0")] pub use iter :: IntoIter ;}

macro_rules! repeat_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function repeat in module {}", module_path!());
    };
}

mkfn!{
    repeat_introspect!();
    # [doc = " Creates an array of type `[T; N]` by repeatedly cloning a value."] # [doc = ""] # [doc = " This is the same as `[val; N]`, but it also works for types that do not"] # [doc = " implement [`Copy`]."] # [doc = ""] # [doc = " The provided value will be used as an element of the resulting array and"] # [doc = " will be cloned N - 1 times to fill up the rest. If N is zero, the value"] # [doc = " will be dropped."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Creating multiple copies of a `String`:"] # [doc = " ```rust"] # [doc = " use std::array;"] # [doc = ""] # [doc = " let string = \"Hello there!\".to_string();"] # [doc = " let strings = array::repeat(string);"] # [doc = " assert_eq!(strings, [\"Hello there!\", \"Hello there!\"]);"] # [doc = " ```"] # [inline] # [must_use = "cloning is often expensive and is not expected to have side effects"] # [stable (feature = "array_repeat" , since = "1.91.0")] pub fn repeat < T : Clone , const N : usize > (val : T) -> [T ; N] { from_trusted_iterator (repeat_n (val , N)) }
}

macro_rules! from_fn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_fn in module {}", module_path!());
    };
}

mkfn!{
    from_fn_introspect!();
    # [doc = " Creates an array where each element is produced by calling `f` with"] # [doc = " that element's index while walking forward through the array."] # [doc = ""] # [doc = " This is essentially the same as writing"] # [doc = " ```text"] # [doc = " [f(0), f(1), f(2), …, f(N - 2), f(N - 1)]"] # [doc = " ```"] # [doc = " and is similar to `(0..i).map(f)`, just for arrays not iterators."] # [doc = ""] # [doc = " If `N == 0`, this produces an empty array without ever calling `f`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " // type inference is helping us here, the way `from_fn` knows how many"] # [doc = " // elements to produce is the length of array down there: only arrays of"] # [doc = " // equal lengths can be compared, so the const generic parameter `N` is"] # [doc = " // inferred to be 5, thus creating array of 5 elements."] # [doc = ""] # [doc = " let array = core::array::from_fn(|i| i);"] # [doc = " // indexes are:    0  1  2  3  4"] # [doc = " assert_eq!(array, [0, 1, 2, 3, 4]);"] # [doc = ""] # [doc = " let array2: [usize; 8] = core::array::from_fn(|i| i * 2);"] # [doc = " // indexes are:     0  1  2  3  4  5   6   7"] # [doc = " assert_eq!(array2, [0, 2, 4, 6, 8, 10, 12, 14]);"] # [doc = ""] # [doc = " let bool_arr = core::array::from_fn::<_, 5, _>(|i| i % 2 == 0);"] # [doc = " // indexes are:       0     1      2     3      4"] # [doc = " assert_eq!(bool_arr, [true, false, true, false, true]);"] # [doc = " ```"] # [doc = ""] # [doc = " You can also capture things, for example to create an array full of clones"] # [doc = " where you can't just use `[item; N]` because it's not `Copy`:"] # [doc = " ```"] # [doc = " # // TBH `array::repeat` would be better for this, but it's not stable yet."] # [doc = " let my_string = String::from(\"Hello\");"] # [doc = " let clones: [String; 42] = std::array::from_fn(|_| my_string.clone());"] # [doc = " assert!(clones.iter().all(|x| *x == my_string));"] # [doc = " ```"] # [doc = ""] # [doc = " The array is generated in ascending index order, starting from the front"] # [doc = " and going towards the back, so you can use closures with mutable state:"] # [doc = " ```"] # [doc = " let mut state = 1;"] # [doc = " let a = std::array::from_fn(|_| { let x = state; state *= 2; x });"] # [doc = " assert_eq!(a, [1, 2, 4, 8, 16, 32]);"] # [doc = " ```"] # [inline] # [stable (feature = "array_from_fn" , since = "1.63.0")] pub fn from_fn < T , const N : usize , F > (f : F) -> [T ; N] where F : FnMut (usize) -> T , { try_from_fn (NeverShortCircuit :: wrap_mut_1 (f)) . 0 }
}

macro_rules! try_from_fn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_from_fn in module {}", module_path!());
    };
}

mkfn!{
    try_from_fn_introspect!();
    # [doc = " Creates an array `[T; N]` where each fallible array element `T` is returned by the `cb` call."] # [doc = " Unlike [`from_fn`], where the element creation can't fail, this version will return an error"] # [doc = " if any element creation was unsuccessful."] # [doc = ""] # [doc = " The return type of this function depends on the return type of the closure."] # [doc = " If you return `Result<T, E>` from the closure, you'll get a `Result<[T; N], E>`."] # [doc = " If you return `Option<T>` from the closure, you'll get an `Option<[T; N]>`."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " * `cb`: Callback where the passed argument is the current array index."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![feature(array_try_from_fn)]"] # [doc = ""] # [doc = " let array: Result<[u8; 5], _> = std::array::try_from_fn(|i| i.try_into());"] # [doc = " assert_eq!(array, Ok([0, 1, 2, 3, 4]));"] # [doc = ""] # [doc = " let array: Result<[i8; 200], _> = std::array::try_from_fn(|i| i.try_into());"] # [doc = " assert!(array.is_err());"] # [doc = ""] # [doc = " let array: Option<[_; 4]> = std::array::try_from_fn(|i| i.checked_add(100));"] # [doc = " assert_eq!(array, Some([100, 101, 102, 103]));"] # [doc = ""] # [doc = " let array: Option<[_; 4]> = std::array::try_from_fn(|i| i.checked_sub(100));"] # [doc = " assert_eq!(array, None);"] # [doc = " ```"] # [inline] # [unstable (feature = "array_try_from_fn" , issue = "89379")] pub fn try_from_fn < R , const N : usize , F > (cb : F) -> ChangeOutputType < R , [R :: Output ; N] > where F : FnMut (usize) -> R , R : Try , R :: Residual : Residual < [R :: Output ; N] > , { let mut array = [const { MaybeUninit :: uninit () } ; N] ; match try_from_fn_erased (& mut array , cb) { ControlFlow :: Break (r) => FromResidual :: from_residual (r) , ControlFlow :: Continue (()) => { try { unsafe { MaybeUninit :: array_assume_init (array) } } } } }
}

macro_rules! from_ref_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_ref in module {}", module_path!());
    };
}

mkfn!{
    from_ref_introspect!();
    # [doc = " Converts a reference to `T` into a reference to an array of length 1 (without copying)."] # [stable (feature = "array_from_ref" , since = "1.53.0")] # [rustc_const_stable (feature = "const_array_from_ref_shared" , since = "1.63.0")] pub const fn from_ref < T > (s : & T) -> & [T ; 1] { unsafe { & * (s as * const T) . cast :: < [T ; 1] > () } }
}

macro_rules! from_mut_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_mut in module {}", module_path!());
    };
}

mkfn!{
    from_mut_introspect!();
    # [doc = " Converts a mutable reference to `T` into a mutable reference to an array of length 1 (without copying)."] # [stable (feature = "array_from_ref" , since = "1.53.0")] # [rustc_const_stable (feature = "const_array_from_ref" , since = "1.83.0")] pub const fn from_mut < T > (s : & mut T) -> & mut [T ; 1] { unsafe { & mut * (s as * mut T) . cast :: < [T ; 1] > () } }
}
mkitem!{mkstruct!{# [doc = " The error type returned when a conversion from a slice to an array fails."] # [stable (feature = "try_from" , since = "1.34.0")] # [derive (Debug , Copy , Clone)] pub struct TryFromSliceError (()) ;}}
mkitem!{mkimpl!{# [stable (feature = "core_array" , since = "1.35.0")] impl fmt :: Display for TryFromSliceError { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { "could not convert slice to array" . fmt (f) } }}}
mkitem!{mkimpl!{# [stable (feature = "try_from" , since = "1.34.0")] impl Error for TryFromSliceError { }}}
mkitem!{# [stable (feature = "try_from_slice_error" , since = "1.36.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl const From < Infallible > for TryFromSliceError { fn from (x : Infallible) -> TryFromSliceError { match x { } } }}
mkitem!{# [stable (feature = "rust1" , since = "1.0.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl < T , const N : usize > const AsRef < [T] > for [T ; N] { # [inline] fn as_ref (& self) -> & [T] { & self [..] } }}
mkitem!{# [stable (feature = "rust1" , since = "1.0.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl < T , const N : usize > const AsMut < [T] > for [T ; N] { # [inline] fn as_mut (& mut self) -> & mut [T] { & mut self [..] } }}
mkitem!{# [stable (feature = "array_borrow" , since = "1.4.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl < T , const N : usize > const Borrow < [T] > for [T ; N] { fn borrow (& self) -> & [T] { self } }}
mkitem!{# [stable (feature = "array_borrow" , since = "1.4.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl < T , const N : usize > const BorrowMut < [T] > for [T ; N] { fn borrow_mut (& mut self) -> & mut [T] { self } }}
mkitem!{# [doc = " Tries to create an array `[T; N]` by copying from a slice `&[T]`."] # [doc = " Succeeds if `slice.len() == N`."] # [doc = ""] # [doc = " ```"] # [doc = " let bytes: [u8; 3] = [1, 0, 2];"] # [doc = ""] # [doc = " let bytes_head: [u8; 2] = <[u8; 2]>::try_from(&bytes[0..2]).unwrap();"] # [doc = " assert_eq!(1, u16::from_le_bytes(bytes_head));"] # [doc = ""] # [doc = " let bytes_tail: [u8; 2] = bytes[1..3].try_into().unwrap();"] # [doc = " assert_eq!(512, u16::from_le_bytes(bytes_tail));"] # [doc = " ```"] # [stable (feature = "try_from" , since = "1.34.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl < T , const N : usize > const TryFrom <& [T] > for [T ; N] where T : Copy , { type Error = TryFromSliceError ; # [inline] fn try_from (slice : & [T]) -> Result < [T ; N] , TryFromSliceError > { <& Self >:: try_from (slice) . copied () } }}
mkitem!{# [doc = " Tries to create an array `[T; N]` by copying from a mutable slice `&mut [T]`."] # [doc = " Succeeds if `slice.len() == N`."] # [doc = ""] # [doc = " ```"] # [doc = " let mut bytes: [u8; 3] = [1, 0, 2];"] # [doc = ""] # [doc = " let bytes_head: [u8; 2] = <[u8; 2]>::try_from(&mut bytes[0..2]).unwrap();"] # [doc = " assert_eq!(1, u16::from_le_bytes(bytes_head));"] # [doc = ""] # [doc = " let bytes_tail: [u8; 2] = (&mut bytes[1..3]).try_into().unwrap();"] # [doc = " assert_eq!(512, u16::from_le_bytes(bytes_tail));"] # [doc = " ```"] # [stable (feature = "try_from_mut_slice_to_array" , since = "1.59.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl < T , const N : usize > const TryFrom <& mut [T] > for [T ; N] where T : Copy , { type Error = TryFromSliceError ; # [inline] fn try_from (slice : & mut [T]) -> Result < [T ; N] , TryFromSliceError > { < Self >:: try_from (&* slice) } }}
mkitem!{# [doc = " Tries to create an array ref `&[T; N]` from a slice ref `&[T]`. Succeeds if"] # [doc = " `slice.len() == N`."] # [doc = ""] # [doc = " ```"] # [doc = " let bytes: [u8; 3] = [1, 0, 2];"] # [doc = ""] # [doc = " let bytes_head: &[u8; 2] = <&[u8; 2]>::try_from(&bytes[0..2]).unwrap();"] # [doc = " assert_eq!(1, u16::from_le_bytes(*bytes_head));"] # [doc = ""] # [doc = " let bytes_tail: &[u8; 2] = bytes[1..3].try_into().unwrap();"] # [doc = " assert_eq!(512, u16::from_le_bytes(*bytes_tail));"] # [doc = " ```"] # [stable (feature = "try_from" , since = "1.34.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl <'a , T , const N : usize > const TryFrom <&'a [T] > for &'a [T ; N] { type Error = TryFromSliceError ; # [inline] fn try_from (slice : &'a [T]) -> Result <&'a [T ; N] , TryFromSliceError > { slice . as_array () . ok_or (TryFromSliceError (())) } }}
mkitem!{# [doc = " Tries to create a mutable array ref `&mut [T; N]` from a mutable slice ref"] # [doc = " `&mut [T]`. Succeeds if `slice.len() == N`."] # [doc = ""] # [doc = " ```"] # [doc = " let mut bytes: [u8; 3] = [1, 0, 2];"] # [doc = ""] # [doc = " let bytes_head: &mut [u8; 2] = <&mut [u8; 2]>::try_from(&mut bytes[0..2]).unwrap();"] # [doc = " assert_eq!(1, u16::from_le_bytes(*bytes_head));"] # [doc = ""] # [doc = " let bytes_tail: &mut [u8; 2] = (&mut bytes[1..3]).try_into().unwrap();"] # [doc = " assert_eq!(512, u16::from_le_bytes(*bytes_tail));"] # [doc = " ```"] # [stable (feature = "try_from" , since = "1.34.0")] # [rustc_const_unstable (feature = "const_convert" , issue = "143773")] impl <'a , T , const N : usize > const TryFrom <&'a mut [T] > for &'a mut [T ; N] { type Error = TryFromSliceError ; # [inline] fn try_from (slice : &'a mut [T]) -> Result <&'a mut [T ; N] , TryFromSliceError > { slice . as_mut_array () . ok_or (TryFromSliceError (())) } }}
mkitem!{mkimpl!{# [doc = " The hash of an array is the same as that of the corresponding slice,"] # [doc = " as required by the `Borrow` implementation."] # [doc = ""] # [doc = " ```"] # [doc = " use std::hash::BuildHasher;"] # [doc = ""] # [doc = " let b = std::hash::RandomState::new();"] # [doc = " let a: [u8; 3] = [0xa8, 0x3c, 0x09];"] # [doc = " let s: &[u8] = &[0xa8, 0x3c, 0x09];"] # [doc = " assert_eq!(b.hash_one(a), b.hash_one(s));"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : Hash , const N : usize > Hash for [T ; N] { fn hash < H : hash :: Hasher > (& self , state : & mut H) { Hash :: hash (& self [..] , state) } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < T : fmt :: Debug , const N : usize > fmt :: Debug for [T ; N] { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& & self [..] , f) } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T , const N : usize > IntoIterator for & 'a [T ; N] { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }}}
mkitem!{mkimpl!{# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T , const N : usize > IntoIterator for & 'a mut [T ; N] { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> IterMut < 'a , T > { self . iter_mut () } }}}
mkitem!{# [stable (feature = "index_trait_on_arrays" , since = "1.50.0")] # [rustc_const_unstable (feature = "const_index" , issue = "143775")] impl < T , I , const N : usize > const Index < I > for [T ; N] where [T] : [const] Index < I >, { type Output = < [T] as Index < I >>:: Output ; # [inline] fn index (& self , index : I) -> & Self :: Output { Index :: index (self as & [T] , index) } }}
mkitem!{# [stable (feature = "index_trait_on_arrays" , since = "1.50.0")] # [rustc_const_unstable (feature = "const_index" , issue = "143775")] impl < T , I , const N : usize > const IndexMut < I > for [T ; N] where [T] : [const] IndexMut < I >, { # [inline] fn index_mut (& mut self , index : I) -> & mut Self :: Output { IndexMut :: index_mut (self as & mut [T] , index) } }}
mkitem!{mkimpl!{# [doc = " Implements comparison of arrays [lexicographically](Ord#lexicographical-comparison)."] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : PartialOrd , const N : usize > PartialOrd for [T ; N] { # [inline] fn partial_cmp (& self , other : & [T ; N]) -> Option < Ordering > { PartialOrd :: partial_cmp (& & self [..] , & & other [..]) } # [inline] fn lt (& self , other : & [T ; N]) -> bool { PartialOrd :: lt (& & self [..] , & & other [..]) } # [inline] fn le (& self , other : & [T ; N]) -> bool { PartialOrd :: le (& & self [..] , & & other [..]) } # [inline] fn ge (& self , other : & [T ; N]) -> bool { PartialOrd :: ge (& & self [..] , & & other [..]) } # [inline] fn gt (& self , other : & [T ; N]) -> bool { PartialOrd :: gt (& & self [..] , & & other [..]) } }}}
mkitem!{mkimpl!{# [doc = " Implements comparison of arrays [lexicographically](Ord#lexicographical-comparison)."] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : Ord , const N : usize > Ord for [T ; N] { # [inline] fn cmp (& self , other : & [T ; N]) -> Ordering { Ord :: cmp (& & self [..] , & & other [..]) } }}}
mkitem!{mkimpl!{# [stable (feature = "copy_clone_array_lib" , since = "1.58.0")] impl < T : Copy , const N : usize > Copy for [T ; N] { }}}
mkitem!{mkimpl!{# [stable (feature = "copy_clone_array_lib" , since = "1.58.0")] impl < T : Clone , const N : usize > Clone for [T ; N] { # [inline] fn clone (& self) -> Self { SpecArrayClone :: clone (self) } # [inline] fn clone_from (& mut self , other : & Self) { self . clone_from_slice (other) ; } }}}
mkitem!{mktrait!{trait SpecArrayClone : Clone { fn clone < const N : usize > (array : & [Self ; N]) -> [Self ; N] ; }}}
mkitem!{mkimpl!{impl < T : Clone > SpecArrayClone for T { # [inline] default fn clone < const N : usize > (array : & [T ; N]) -> [T ; N] { from_trusted_iterator (array . iter () . cloned ()) } }}}
mkitem!{mkimpl!{impl < T : Copy > SpecArrayClone for T { # [inline] fn clone < const N : usize > (array : & [T ; N]) -> [T ; N] { * array } }}}
mkitem!{macro_rules ! array_impl_default { { $ n : expr , $ t : ident $ ($ ts : ident) * } => { # [stable (since = "1.4.0" , feature = "array_default")] impl < T > Default for [T ; $ n] where T : Default { fn default () -> [T ; $ n] { [$ t :: default () , $ ($ ts :: default ()) ,*] } } array_impl_default ! { ($ n - 1) , $ ($ ts) * } } ; { $ n : expr , } => { # [stable (since = "1.4.0" , feature = "array_default")] impl < T > Default for [T ; $ n] { fn default () -> [T ; $ n] { [] } } } ; }}
mkitem!{array_impl_default ! { 32 , T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T T }}
mkitem!{mkimpl!{impl < T , const N : usize > [T ; N] { # [doc = " Returns an array of the same size as `self`, with function `f` applied to each element"] # [doc = " in order."] # [doc = ""] # [doc = " If you don't necessarily need a new fixed-size array, consider using"] # [doc = " [`Iterator::map`] instead."] # [doc = ""] # [doc = ""] # [doc = " # Note on performance and stack usage"] # [doc = ""] # [doc = " Unfortunately, usages of this method are currently not always optimized"] # [doc = " as well as they could be. This mainly concerns large arrays, as mapping"] # [doc = " over small arrays seem to be optimized just fine. Also note that in"] # [doc = " debug mode (i.e. without any optimizations), this method can use a lot"] # [doc = " of stack space (a few times the size of the array or more)."] # [doc = ""] # [doc = " Therefore, in performance-critical code, try to avoid using this method"] # [doc = " on large arrays or check the emitted code. Also try to avoid chained"] # [doc = " maps (e.g. `arr.map(...).map(...)`)."] # [doc = ""] # [doc = " In many cases, you can instead use [`Iterator::map`] by calling `.iter()`"] # [doc = " or `.into_iter()` on your array. `[T; N]::map` is only necessary if you"] # [doc = " really need a new array of the same size as the result. Rust's lazy"] # [doc = " iterators tend to get optimized very well."] # [doc = ""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let x = [1, 2, 3];"] # [doc = " let y = x.map(|v| v + 1);"] # [doc = " assert_eq!(y, [2, 3, 4]);"] # [doc = ""] # [doc = " let x = [1, 2, 3];"] # [doc = " let mut temp = 0;"] # [doc = " let y = x.map(|v| { temp += 1; v * temp });"] # [doc = " assert_eq!(y, [1, 4, 9]);"] # [doc = ""] # [doc = " let x = [\"Ferris\", \"Bueller's\", \"Day\", \"Off\"];"] # [doc = " let y = x.map(|v| v.len());"] # [doc = " assert_eq!(y, [6, 9, 3, 3]);"] # [doc = " ```"] # [must_use] # [stable (feature = "array_map" , since = "1.55.0")] pub fn map < F , U > (self , f : F) -> [U ; N] where F : FnMut (T) -> U , { self . try_map (NeverShortCircuit :: wrap_mut_1 (f)) . 0 } # [doc = " A fallible function `f` applied to each element on array `self` in order to"] # [doc = " return an array the same size as `self` or the first error encountered."] # [doc = ""] # [doc = " The return type of this function depends on the return type of the closure."] # [doc = " If you return `Result<T, E>` from the closure, you'll get a `Result<[T; N], E>`."] # [doc = " If you return `Option<T>` from the closure, you'll get an `Option<[T; N]>`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(array_try_map)]"] # [doc = ""] # [doc = " let a = [\"1\", \"2\", \"3\"];"] # [doc = " let b = a.try_map(|v| v.parse::<u32>()).unwrap().map(|v| v + 1);"] # [doc = " assert_eq!(b, [2, 3, 4]);"] # [doc = ""] # [doc = " let a = [\"1\", \"2a\", \"3\"];"] # [doc = " let b = a.try_map(|v| v.parse::<u32>());"] # [doc = " assert!(b.is_err());"] # [doc = ""] # [doc = " use std::num::NonZero;"] # [doc = ""] # [doc = " let z = [1, 2, 0, 3, 4];"] # [doc = " assert_eq!(z.try_map(NonZero::new), None);"] # [doc = ""] # [doc = " let a = [1, 2, 3];"] # [doc = " let b = a.try_map(NonZero::new);"] # [doc = " let c = b.map(|x| x.map(NonZero::get));"] # [doc = " assert_eq!(c, Some(a));"] # [doc = " ```"] # [unstable (feature = "array_try_map" , issue = "79711")] pub fn try_map < R > (self , f : impl FnMut (T) -> R) -> ChangeOutputType < R , [R :: Output ; N] > where R : Try < Residual : Residual < [R :: Output ; N] > > , { drain_array_with (self , | iter | try_from_trusted_iterator (iter . map (f))) } # [doc = " Returns a slice containing the entire array. Equivalent to `&s[..]`."] # [stable (feature = "array_as_slice" , since = "1.57.0")] # [rustc_const_stable (feature = "array_as_slice" , since = "1.57.0")] pub const fn as_slice (& self) -> & [T] { self } # [doc = " Returns a mutable slice containing the entire array. Equivalent to"] # [doc = " `&mut s[..]`."] # [stable (feature = "array_as_slice" , since = "1.57.0")] # [rustc_const_stable (feature = "const_array_as_mut_slice" , since = "1.89.0")] pub const fn as_mut_slice (& mut self) -> & mut [T] { self } # [doc = " Borrows each element and returns an array of references with the same"] # [doc = " size as `self`."] # [doc = ""] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let floats = [3.1, 2.7, -1.0];"] # [doc = " let float_refs: [&f64; 3] = floats.each_ref();"] # [doc = " assert_eq!(float_refs, [&3.1, &2.7, &-1.0]);"] # [doc = " ```"] # [doc = ""] # [doc = " This method is particularly useful if combined with other methods, like"] # [doc = " [`map`](#method.map). This way, you can avoid moving the original"] # [doc = " array if its elements are not [`Copy`]."] # [doc = ""] # [doc = " ```"] # [doc = " let strings = [\"Ferris\".to_string(), \"♥\".to_string(), \"Rust\".to_string()];"] # [doc = " let is_ascii = strings.each_ref().map(|s| s.is_ascii());"] # [doc = " assert_eq!(is_ascii, [true, false, true]);"] # [doc = ""] # [doc = " // We can still access the original array: it has not been moved."] # [doc = " assert_eq!(strings.len(), 3);"] # [doc = " ```"] # [stable (feature = "array_methods" , since = "1.77.0")] # [rustc_const_stable (feature = "const_array_each_ref" , since = "1.91.0")] pub const fn each_ref (& self) -> [& T ; N] { let mut buf = [null :: < T > () ; N] ; let mut i = 0 ; while i < N { buf [i] = & raw const self [i] ; i += 1 ; } unsafe { transmute_unchecked (buf) } } # [doc = " Borrows each element mutably and returns an array of mutable references"] # [doc = " with the same size as `self`."] # [doc = ""] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = ""] # [doc = " let mut floats = [3.1, 2.7, -1.0];"] # [doc = " let float_refs: [&mut f64; 3] = floats.each_mut();"] # [doc = " *float_refs[0] = 0.0;"] # [doc = " assert_eq!(float_refs, [&mut 0.0, &mut 2.7, &mut -1.0]);"] # [doc = " assert_eq!(floats, [0.0, 2.7, -1.0]);"] # [doc = " ```"] # [stable (feature = "array_methods" , since = "1.77.0")] # [rustc_const_stable (feature = "const_array_each_ref" , since = "1.91.0")] pub const fn each_mut (& mut self) -> [& mut T ; N] { let mut buf = [null_mut :: < T > () ; N] ; let mut i = 0 ; while i < N { buf [i] = & raw mut self [i] ; i += 1 ; } unsafe { transmute_unchecked (buf) } } # [doc = " Divides one array reference into two at an index."] # [doc = ""] # [doc = " The first will contain all indices from `[0, M)` (excluding"] # [doc = " the index `M` itself) and the second will contain all"] # [doc = " indices from `[M, N)` (excluding the index `N` itself)."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `M > N`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(split_array)]"] # [doc = ""] # [doc = " let v = [1, 2, 3, 4, 5, 6];"] # [doc = ""] # [doc = " {"] # [doc = "    let (left, right) = v.split_array_ref::<0>();"] # [doc = "    assert_eq!(left, &[]);"] # [doc = "    assert_eq!(right, &[1, 2, 3, 4, 5, 6]);"] # [doc = " }"] # [doc = ""] # [doc = " {"] # [doc = "     let (left, right) = v.split_array_ref::<2>();"] # [doc = "     assert_eq!(left, &[1, 2]);"] # [doc = "     assert_eq!(right, &[3, 4, 5, 6]);"] # [doc = " }"] # [doc = ""] # [doc = " {"] # [doc = "     let (left, right) = v.split_array_ref::<6>();"] # [doc = "     assert_eq!(left, &[1, 2, 3, 4, 5, 6]);"] # [doc = "     assert_eq!(right, &[]);"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "split_array" , reason = "return type should have array as 2nd element" , issue = "90091")] # [inline] pub fn split_array_ref < const M : usize > (& self) -> (& [T ; M] , & [T]) { self . split_first_chunk :: < M > () . unwrap () } # [doc = " Divides one mutable array reference into two at an index."] # [doc = ""] # [doc = " The first will contain all indices from `[0, M)` (excluding"] # [doc = " the index `M` itself) and the second will contain all"] # [doc = " indices from `[M, N)` (excluding the index `N` itself)."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `M > N`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(split_array)]"] # [doc = ""] # [doc = " let mut v = [1, 0, 3, 0, 5, 6];"] # [doc = " let (left, right) = v.split_array_mut::<2>();"] # [doc = " assert_eq!(left, &mut [1, 0][..]);"] # [doc = " assert_eq!(right, &mut [3, 0, 5, 6]);"] # [doc = " left[1] = 2;"] # [doc = " right[1] = 4;"] # [doc = " assert_eq!(v, [1, 2, 3, 4, 5, 6]);"] # [doc = " ```"] # [unstable (feature = "split_array" , reason = "return type should have array as 2nd element" , issue = "90091")] # [inline] pub fn split_array_mut < const M : usize > (& mut self) -> (& mut [T ; M] , & mut [T]) { self . split_first_chunk_mut :: < M > () . unwrap () } # [doc = " Divides one array reference into two at an index from the end."] # [doc = ""] # [doc = " The first will contain all indices from `[0, N - M)` (excluding"] # [doc = " the index `N - M` itself) and the second will contain all"] # [doc = " indices from `[N - M, N)` (excluding the index `N` itself)."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `M > N`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(split_array)]"] # [doc = ""] # [doc = " let v = [1, 2, 3, 4, 5, 6];"] # [doc = ""] # [doc = " {"] # [doc = "    let (left, right) = v.rsplit_array_ref::<0>();"] # [doc = "    assert_eq!(left, &[1, 2, 3, 4, 5, 6]);"] # [doc = "    assert_eq!(right, &[]);"] # [doc = " }"] # [doc = ""] # [doc = " {"] # [doc = "     let (left, right) = v.rsplit_array_ref::<2>();"] # [doc = "     assert_eq!(left, &[1, 2, 3, 4]);"] # [doc = "     assert_eq!(right, &[5, 6]);"] # [doc = " }"] # [doc = ""] # [doc = " {"] # [doc = "     let (left, right) = v.rsplit_array_ref::<6>();"] # [doc = "     assert_eq!(left, &[]);"] # [doc = "     assert_eq!(right, &[1, 2, 3, 4, 5, 6]);"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "split_array" , reason = "return type should have array as 2nd element" , issue = "90091")] # [inline] pub fn rsplit_array_ref < const M : usize > (& self) -> (& [T] , & [T ; M]) { self . split_last_chunk :: < M > () . unwrap () } # [doc = " Divides one mutable array reference into two at an index from the end."] # [doc = ""] # [doc = " The first will contain all indices from `[0, N - M)` (excluding"] # [doc = " the index `N - M` itself) and the second will contain all"] # [doc = " indices from `[N - M, N)` (excluding the index `N` itself)."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `M > N`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(split_array)]"] # [doc = ""] # [doc = " let mut v = [1, 0, 3, 0, 5, 6];"] # [doc = " let (left, right) = v.rsplit_array_mut::<4>();"] # [doc = " assert_eq!(left, &mut [1, 0]);"] # [doc = " assert_eq!(right, &mut [3, 0, 5, 6][..]);"] # [doc = " left[1] = 2;"] # [doc = " right[1] = 4;"] # [doc = " assert_eq!(v, [1, 2, 3, 4, 5, 6]);"] # [doc = " ```"] # [unstable (feature = "split_array" , reason = "return type should have array as 2nd element" , issue = "90091")] # [inline] pub fn rsplit_array_mut < const M : usize > (& mut self) -> (& mut [T] , & mut [T ; M]) { self . split_last_chunk_mut :: < M > () . unwrap () } }}}

macro_rules! from_trusted_iterator_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function from_trusted_iterator in module {}", module_path!());
    };
}

mkfn!{
    from_trusted_iterator_introspect!();
    # [doc = " Populate an array from the first `N` elements of `iter`"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If the iterator doesn't actually have enough items."] # [doc = ""] # [doc = " By depending on `TrustedLen`, however, we can do that check up-front (where"] # [doc = " it easily optimizes away) so it doesn't impact the loop that fills the array."] # [inline] fn from_trusted_iterator < T , const N : usize > (iter : impl UncheckedIterator < Item = T >) -> [T ; N] { try_from_trusted_iterator (iter . map (NeverShortCircuit)) . 0 }
}

macro_rules! try_from_trusted_iterator_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_from_trusted_iterator in module {}", module_path!());
    };
}

mkfn!{
    try_from_trusted_iterator_introspect!();
    # [inline] fn try_from_trusted_iterator < T , R , const N : usize > (iter : impl UncheckedIterator < Item = R > ,) -> ChangeOutputType < R , [T ; N] > where R : Try < Output = T > , R :: Residual : Residual < [T ; N] > , { assert ! (iter . size_hint () . 0 >= N) ; fn next < T > (mut iter : impl UncheckedIterator < Item = T >) -> impl FnMut (usize) -> T { move | _ | { unsafe { iter . next_unchecked () } } } try_from_fn (next (iter)) }
}

macro_rules! try_from_fn_erased_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_from_fn_erased in module {}", module_path!());
    };
}

mkfn!{
    try_from_fn_erased_introspect!();
    # [doc = " Version of [`try_from_fn`] using a passed-in slice in order to avoid"] # [doc = " needing to monomorphize for every array length."] # [doc = ""] # [doc = " This takes a generator rather than an iterator so that *at the type level*"] # [doc = " it never needs to worry about running out of items.  When combined with"] # [doc = " an infallible `Try` type, that means the loop canonicalizes easily, allowing"] # [doc = " it to optimize well."] # [doc = ""] # [doc = " It would be *possible* to unify this and [`iter_next_chunk_erased`] into one"] # [doc = " function that does the union of both things, but last time it was that way"] # [doc = " it resulted in poor codegen from the \"are there enough source items?\" checks"] # [doc = " not optimizing away.  So if you give it a shot, make sure to watch what"] # [doc = " happens in the codegen tests."] # [inline] fn try_from_fn_erased < T , R > (buffer : & mut [MaybeUninit < T >] , mut generator : impl FnMut (usize) -> R ,) -> ControlFlow < R :: Residual > where R : Try < Output = T > , { let mut guard = Guard { array_mut : buffer , initialized : 0 } ; while guard . initialized < guard . array_mut . len () { let item = generator (guard . initialized) . branch () ? ; unsafe { guard . push_unchecked (item) } ; } mem :: forget (guard) ; ControlFlow :: Continue (()) }
}
mkitem!{mkstruct!{# [doc = " Panic guard for incremental initialization of arrays."] # [doc = ""] # [doc = " Disarm the guard with `mem::forget` once the array has been initialized."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " All write accesses to this structure are unsafe and must maintain a correct"] # [doc = " count of `initialized` elements."] # [doc = ""] # [doc = " To minimize indirection fields are still pub but callers should at least use"] # [doc = " `push_unchecked` to signal that something unsafe is going on."] struct Guard < 'a , T > { # [doc = " The array to be initialized."] pub array_mut : & 'a mut [MaybeUninit < T >] , # [doc = " The number of items that have been initialized so far."] pub initialized : usize , }}}
mkitem!{mkimpl!{impl < T > Guard < '_ , T > { # [doc = " Adds an item to the array and updates the initialized item counter."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " No more than N elements must be initialized."] # [inline] pub (crate) unsafe fn push_unchecked (& mut self , item : T) { unsafe { self . array_mut . get_unchecked_mut (self . initialized) . write (item) ; self . initialized = self . initialized . unchecked_add (1) ; } } }}}
mkitem!{mkimpl!{impl < T > Drop for Guard < '_ , T > { # [inline] fn drop (& mut self) { debug_assert ! (self . initialized <= self . array_mut . len ()) ; unsafe { self . array_mut . get_unchecked_mut (.. self . initialized) . assume_init_drop () ; } } }}}

macro_rules! iter_next_chunk_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iter_next_chunk in module {}", module_path!());
    };
}

mkfn!{
    iter_next_chunk_introspect!();
    # [doc = " Pulls `N` items from `iter` and returns them as an array. If the iterator"] # [doc = " yields fewer than `N` items, `Err` is returned containing an iterator over"] # [doc = " the already yielded items."] # [doc = ""] # [doc = " Since the iterator is passed as a mutable reference and this function calls"] # [doc = " `next` at most `N` times, the iterator can still be used afterwards to"] # [doc = " retrieve the remaining items."] # [doc = ""] # [doc = " If `iter.next()` panicks, all items already yielded by the iterator are"] # [doc = " dropped."] # [doc = ""] # [doc = " Used for [`Iterator::next_chunk`]."] # [inline] pub (crate) fn iter_next_chunk < T , const N : usize > (iter : & mut impl Iterator < Item = T > ,) -> Result < [T ; N] , IntoIter < T , N > > { let mut array = [const { MaybeUninit :: uninit () } ; N] ; let r = iter_next_chunk_erased (& mut array , iter) ; match r { Ok (()) => { Ok (unsafe { MaybeUninit :: array_assume_init (array) }) } Err (initialized) => { Err (unsafe { IntoIter :: new_unchecked (array , 0 .. initialized) }) } } }
}

macro_rules! iter_next_chunk_erased_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iter_next_chunk_erased in module {}", module_path!());
    };
}

mkfn!{
    iter_next_chunk_erased_introspect!();
    # [doc = " Version of [`iter_next_chunk`] using a passed-in slice in order to avoid"] # [doc = " needing to monomorphize for every array length."] # [doc = ""] # [doc = " Unfortunately this loop has two exit conditions, the buffer filling up"] # [doc = " or the iterator running out of items, making it tend to optimize poorly."] # [inline] fn iter_next_chunk_erased < T > (buffer : & mut [MaybeUninit < T >] , iter : & mut impl Iterator < Item = T > ,) -> Result < () , usize > { let mut guard = Guard { array_mut : buffer , initialized : 0 } ; while guard . initialized < guard . array_mut . len () { let Some (item) = iter . next () else { let initialized = guard . initialized ; mem :: forget (guard) ; return Err (initialized) ; } ; unsafe { guard . push_unchecked (item) } ; } mem :: forget (guard) ; Ok (()) }
}