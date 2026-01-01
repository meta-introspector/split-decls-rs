/* FP:list.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_USE_0001
/* FP:list.rs-0002 */ use std :: alloc :: Layout ;
/* FP:list.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_USE_0002
/* FP:list.rs-0004 */ use std :: cmp :: Ordering ;
/* FP:list.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_USE_0003
/* FP:list.rs-0006 */ use std :: hash :: { Hash , Hasher } ;
/* FP:list.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_USE_0004
/* FP:list.rs-0008 */ use std :: ops :: Deref ;
/* FP:list.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_USE_0005
/* FP:list.rs-0010 */ use std :: { fmt , iter , mem , ptr , slice } ;
/* FP:list.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_USE_0006
/* FP:list.rs-0012 */ use crate :: rustc_data_structures :: aligned :: { Aligned , align_of } ;
/* FP:list.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_USE_0007
/* FP:list.rs-0014 */ use crate :: rustc_data_structures :: sync :: DynSync ;
/* FP:list.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_USE_0008
/* FP:list.rs-0016 */ use crate :: rustc_serialize :: { Encodable , Encoder } ;
/* FP:list.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_USE_0009
/* FP:list.rs-0018 */ use rustc_type_ir :: FlagComputation ;
/* FP:list.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_USE_0010
/* FP:list.rs-0020 */ use super :: { DebruijnIndex , TyCtxt , TypeFlags } ;
/* FP:list.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_USE_0011
/* FP:list.rs-0022 */ use crate :: arena :: Arena ;
/* FP:list.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_TYPE_0012
/* FP:list.rs-0024 */ # [doc = " `List<T>` is a bit like `&[T]`, but with some critical differences."] # [doc = " - IMPORTANT: Every `List<T>` is *required* to have unique contents. The"] # [doc = "   type's correctness relies on this, *but it does not enforce it*."] # [doc = "   Therefore, any code that creates a `List<T>` must ensure uniqueness"] # [doc = "   itself. In practice this is achieved by interning."] # [doc = " - The length is stored within the `List<T>`, so `&List<Ty>` is a thin"] # [doc = "   pointer."] # [doc = " - Because of this, you cannot get a `List<T>` that is a sub-list of another"] # [doc = "   `List<T>`. You can get a sub-slice `&[T]`, however."] # [doc = " - `List<T>` can be used with `TaggedRef`, which is useful within"] # [doc = "   structs whose size must be minimized."] # [doc = " - Because of the uniqueness assumption, we can use the address of a"] # [doc = "   `List<T>` for faster equality comparisons and hashing."] # [doc = " - `T` must be `Copy`. This lets `List<T>` be stored in a dropless arena and"] # [doc = "   iterators return a `T` rather than a `&T`."] # [doc = " - `T` must not be zero-sized."] pub type List < T > = RawList < () , T > ;
/* FP:list.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_STRUCT_0013
/* FP:list.rs-0026 */ # [doc = " A generic type that can be used to prepend a [`List`] with some header."] # [doc = ""] # [doc = " The header will be ignored for value-based operations like [`PartialEq`],"] # [doc = " [`Hash`] and [`Encodable`]."] # [repr (C)] pub struct RawList < H , T > { skel : ListSkeleton < H , T > , opaque : OpaqueListContents , }
/* FP:list.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_STRUCT_0014
/* FP:list.rs-0028 */ # [doc = " A [`RawList`] without the unsized tail. This type is used for layout computation"] # [doc = " and constructing empty lists."] # [repr (C)] struct ListSkeleton < H , T > { header : H , len : usize , # [doc = " Although this claims to be a zero-length array, in practice `len`"] # [doc = " elements are actually present."] data : [T ; 0] , }
/* FP:list.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0015
/* FP:list.rs-0030 */ impl < T > Default for & List < T > { fn default () -> Self { List :: empty () } }
/* FP:list.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_OTHER_0016
/* FP:list.rs-0032 */ unsafe extern "C" { # [doc = " A dummy type used to force `List` to be unsized while not requiring"] # [doc = " references to it be wide pointers."] type OpaqueListContents ; }
/* FP:list.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0017
/* FP:list.rs-0034 */ impl < H , T > RawList < H , T > { # [inline (always)] pub fn len (& self) -> usize { self . skel . len } # [inline (always)] pub fn as_slice (& self) -> & [T] { self } # [doc = " Allocates a list from `arena` and copies the contents of `slice` into it."] # [doc = ""] # [doc = " WARNING: the contents *must be unique*, such that no list with these"] # [doc = " contents has been previously created. If not, operations such as `eq`"] # [doc = " and `hash` might give incorrect results."] # [doc = ""] # [doc = " Panics if `T` is `Drop`, or `T` is zero-sized, or the slice is empty"] # [doc = " (because the empty list exists statically, and is available via"] # [doc = " `empty()`)."] # [inline] pub (super) fn from_arena < 'tcx > (arena : & 'tcx Arena < 'tcx > , header : H , slice : & [T] ,) -> & 'tcx RawList < H , T > where T : Copy , { assert ! (! mem :: needs_drop ::< T > ()) ; assert ! (size_of ::< T > () != 0) ; assert ! (! slice . is_empty ()) ; let (layout , _offset) = Layout :: new :: < ListSkeleton < H , T > > () . extend (Layout :: for_value :: < [T] > (slice)) . unwrap () ; let mem = arena . dropless . alloc_raw (layout) as * mut RawList < H , T > ; unsafe { (& raw mut (* mem) . skel . header) . write (header) ; (& raw mut (* mem) . skel . len) . write (slice . len ()) ; (& raw mut (* mem) . skel . data) . cast :: < T > () . copy_from_nonoverlapping (slice . as_ptr () , slice . len ()) ; & * mem } } # [inline (always)] pub fn iter (& self) -> < & '_ RawList < H , T > as IntoIterator > :: IntoIter where T : Copy , { self . into_iter () } }
/* FP:list.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0018
/* FP:list.rs-0036 */ impl < 'a , H , T : Copy > rustc_type_ir :: inherent :: SliceLike for & 'a RawList < H , T > { type Item = T ; type IntoIter = iter :: Copied < < & 'a [T] as IntoIterator > :: IntoIter > ; fn iter (self) -> Self :: IntoIter { (* self) . iter () } fn as_slice (& self) -> & [Self :: Item] { (* self) . as_slice () } }
/* FP:list.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_MACRO_0019
/* FP:list.rs-0038 */ macro_rules ! impl_list_empty { ($ header_ty : ty , $ header_init : expr) => { impl < T > RawList <$ header_ty , T > { # [doc = " Returns a reference to the (per header unique, static) empty list."] # [inline (always)] pub fn empty <'a > () -> &'a RawList <$ header_ty , T > { # [repr (align (64))] struct MaxAlign ; static EMPTY : ListSkeleton <$ header_ty , MaxAlign > = ListSkeleton { header : $ header_init , len : 0 , data : [] } ; assert ! (align_of ::< T > () <= align_of ::< MaxAlign > ()) ; unsafe { &* ((& raw const EMPTY) as * const RawList <$ header_ty , T >) } } } } ; }
/* FP:list.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_MACRO_0020
/* FP:list.rs-0040 */ impl_list_empty ! (() , ()) ;
/* FP:list.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0021
/* FP:list.rs-0042 */ impl < H , T : fmt :: Debug > fmt :: Debug for RawList < H , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
/* FP:list.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0022
/* FP:list.rs-0044 */ impl < H , S : Encoder , T : Encodable < S > > Encodable < S > for RawList < H , T > { # [inline] fn encode (& self , s : & mut S) { (* * self) . encode (s) ; } }
/* FP:list.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0023
/* FP:list.rs-0046 */ impl < H , T : PartialEq > PartialEq for RawList < H , T > { # [inline] fn eq (& self , other : & RawList < H , T >) -> bool { ptr :: eq (self , other) } }
/* FP:list.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0024
/* FP:list.rs-0048 */ impl < H , T : Eq > Eq for RawList < H , T > { }
/* FP:list.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0025
/* FP:list.rs-0050 */ impl < H , T > Ord for RawList < H , T > where T : Ord , { fn cmp (& self , other : & RawList < H , T >) -> Ordering { if self == other { Ordering :: Equal } else { < [T] as Ord > :: cmp (& * * self , & * * other) } } }
/* FP:list.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0026
/* FP:list.rs-0052 */ impl < H , T > PartialOrd for RawList < H , T > where T : PartialOrd , { fn partial_cmp (& self , other : & RawList < H , T >) -> Option < Ordering > { if self == other { Some (Ordering :: Equal) } else { < [T] as PartialOrd > :: partial_cmp (& * * self , & * * other) } } }
/* FP:list.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0027
/* FP:list.rs-0054 */ impl < Hdr , T > Hash for RawList < Hdr , T > { # [inline] fn hash < H : Hasher > (& self , s : & mut H) { ptr :: from_ref (self) . hash (s) } }
/* FP:list.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0028
/* FP:list.rs-0056 */ impl < H , T > Deref for RawList < H , T > { type Target = [T] ; # [inline (always)] fn deref (& self) -> & [T] { self . as_ref () } }
/* FP:list.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0029
/* FP:list.rs-0058 */ impl < H , T > AsRef < [T] > for RawList < H , T > { # [inline (always)] fn as_ref (& self) -> & [T] { let data_ptr = (& raw const self . skel . data) . cast :: < T > () ; unsafe { slice :: from_raw_parts (data_ptr , self . skel . len) } } }
/* FP:list.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0030
/* FP:list.rs-0060 */ impl < 'a , H , T : Copy > IntoIterator for & 'a RawList < H , T > { type Item = T ; type IntoIter = iter :: Copied < < & 'a [T] as IntoIterator > :: IntoIter > ; # [inline (always)] fn into_iter (self) -> Self :: IntoIter { self [..] . iter () . copied () } }
/* FP:list.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0031
/* FP:list.rs-0062 */ unsafe impl < H : Sync , T : Sync > Sync for RawList < H , T > { }
/* FP:list.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0032
/* FP:list.rs-0064 */ unsafe impl < H : DynSync , T : DynSync > DynSync for RawList < H , T > { }
/* FP:list.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0033
/* FP:list.rs-0066 */ unsafe impl < H , T > Aligned for RawList < H , T > { const ALIGN : ptr :: Alignment = align_of :: < ListSkeleton < H , T > > () ; }
/* FP:list.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_TYPE_0034
/* FP:list.rs-0068 */ # [doc = " A [`List`] that additionally stores type information inline to speed up"] # [doc = " [`TypeVisitableExt`](super::TypeVisitableExt) operations."] pub type ListWithCachedTypeInfo < T > = RawList < TypeInfo , T > ;
/* FP:list.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0035
/* FP:list.rs-0070 */ impl < T > ListWithCachedTypeInfo < T > { # [inline (always)] pub fn flags (& self) -> TypeFlags { self . skel . header . flags } # [inline (always)] pub fn outer_exclusive_binder (& self) -> DebruijnIndex { self . skel . header . outer_exclusive_binder } }
/* FP:list.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_MACRO_0036
/* FP:list.rs-0072 */ impl_list_empty ! (TypeInfo , TypeInfo :: empty ()) ;
/* FP:list.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_STRUCT_0037
/* FP:list.rs-0074 */ # [doc = " The additional info that is stored in [`ListWithCachedTypeInfo`]."] # [repr (C)] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct TypeInfo { flags : TypeFlags , outer_exclusive_binder : DebruijnIndex , }
/* FP:list.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0038
/* FP:list.rs-0076 */ impl TypeInfo { const fn empty () -> Self { Self { flags : TypeFlags :: empty () , outer_exclusive_binder : super :: INNERMOST } } }
/* FP:list.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_list_IMPL_0039
/* FP:list.rs-0078 */ impl < 'tcx > From < FlagComputation < TyCtxt < 'tcx > > > for TypeInfo { fn from (computation : FlagComputation < TyCtxt < 'tcx > >) -> TypeInfo { TypeInfo { flags : computation . flags , outer_exclusive_binder : computation . outer_exclusive_binder , } } }