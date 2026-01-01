/* FP:ty.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_USE_0001
/* FP:ty.rs-0002 */ use std :: fmt :: { self , Debug , Display , Formatter } ;
/* FP:ty.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_USE_0002
/* FP:ty.rs-0004 */ use std :: ops :: Range ;
/* FP:ty.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_USE_0003
/* FP:ty.rs-0006 */ use serde :: Serialize ;
/* FP:ty.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_USE_0004
/* FP:ty.rs-0008 */ use super :: abi :: ReprOptions ;
/* FP:ty.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_USE_0005
/* FP:ty.rs-0010 */ use super :: mir :: { Body , Mutability , Safety } ;
/* FP:ty.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_USE_0006
/* FP:ty.rs-0012 */ use super :: { DefId , Error , Symbol , with } ;
/* FP:ty.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_USE_0007
/* FP:ty.rs-0014 */ use crate :: abi :: { FnAbi , Layout } ;
/* FP:ty.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_USE_0008
/* FP:ty.rs-0016 */ use crate :: crate_def :: { CrateDef , CrateDefItems , CrateDefType } ;
/* FP:ty.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_USE_0009
/* FP:ty.rs-0018 */ use crate :: mir :: alloc :: { AllocId , read_target_int , read_target_uint } ;
/* FP:ty.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_USE_0010
/* FP:ty.rs-0020 */ use crate :: mir :: mono :: StaticDef ;
/* FP:ty.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_USE_0011
/* FP:ty.rs-0022 */ use crate :: target :: MachineInfo ;
/* FP:ty.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_USE_0012
/* FP:ty.rs-0024 */ use crate :: { Filename , IndexedVal , Opaque } ;
/* FP:ty.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0013
/* FP:ty.rs-0026 */ # [derive (Copy , Clone , Eq , PartialEq , Hash , Serialize)] pub struct Ty (usize) ;
/* FP:ty.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0014
/* FP:ty.rs-0028 */ impl Debug for Ty { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Ty") . field ("id" , & self . 0) . field ("kind" , & self . kind ()) . finish () } }
/* FP:ty.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0015
/* FP:ty.rs-0030 */ # [doc = " Constructors for `Ty`."] impl Ty { # [doc = " Create a new type from a given kind."] pub fn from_rigid_kind (kind : RigidTy) -> Ty { with (| cx | cx . new_rigid_ty (kind)) } # [doc = " Create a new array type."] pub fn try_new_array (elem_ty : Ty , size : u64) -> Result < Ty , Error > { Ok (Ty :: from_rigid_kind (RigidTy :: Array (elem_ty , TyConst :: try_from_target_usize (size) ?))) } # [doc = " Create a new array type from Const length."] pub fn new_array_with_const_len (elem_ty : Ty , len : TyConst) -> Ty { Ty :: from_rigid_kind (RigidTy :: Array (elem_ty , len)) } # [doc = " Create a new pointer type."] pub fn new_ptr (pointee_ty : Ty , mutability : Mutability) -> Ty { Ty :: from_rigid_kind (RigidTy :: RawPtr (pointee_ty , mutability)) } # [doc = " Create a new reference type."] pub fn new_ref (reg : Region , pointee_ty : Ty , mutability : Mutability) -> Ty { Ty :: from_rigid_kind (RigidTy :: Ref (reg , pointee_ty , mutability)) } # [doc = " Create a new pointer type."] pub fn new_tuple (tys : & [Ty]) -> Ty { Ty :: from_rigid_kind (RigidTy :: Tuple (Vec :: from (tys))) } # [doc = " Create a new closure type."] pub fn new_closure (def : ClosureDef , args : GenericArgs) -> Ty { Ty :: from_rigid_kind (RigidTy :: Closure (def , args)) } # [doc = " Create a new coroutine type."] pub fn new_coroutine (def : CoroutineDef , args : GenericArgs) -> Ty { Ty :: from_rigid_kind (RigidTy :: Coroutine (def , args)) } # [doc = " Create a new closure type."] pub fn new_coroutine_closure (def : CoroutineClosureDef , args : GenericArgs) -> Ty { Ty :: from_rigid_kind (RigidTy :: CoroutineClosure (def , args)) } # [doc = " Create a new box type that represents `Box<T>`, for the given inner type `T`."] pub fn new_box (inner_ty : Ty) -> Ty { with (| cx | cx . new_box_ty (inner_ty)) } # [doc = " Create a type representing `usize`."] pub fn usize_ty () -> Ty { Ty :: from_rigid_kind (RigidTy :: Uint (UintTy :: Usize)) } # [doc = " Create a type representing `bool`."] pub fn bool_ty () -> Ty { Ty :: from_rigid_kind (RigidTy :: Bool) } # [doc = " Create a type representing a signed integer."] pub fn signed_ty (inner : IntTy) -> Ty { Ty :: from_rigid_kind (RigidTy :: Int (inner)) } # [doc = " Create a type representing an unsigned integer."] pub fn unsigned_ty (inner : UintTy) -> Ty { Ty :: from_rigid_kind (RigidTy :: Uint (inner)) } # [doc = " Get a type layout."] pub fn layout (self) -> Result < Layout , Error > { with (| cx | cx . ty_layout (self)) } }
/* FP:ty.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0016
/* FP:ty.rs-0032 */ impl Ty { pub fn kind (& self) -> TyKind { with (| context | context . ty_kind (* self)) } }
/* FP:ty.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0017
/* FP:ty.rs-0034 */ # [doc = " Represents a pattern in the type system"] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum Pattern { Range { start : Option < TyConst > , end : Option < TyConst > , include_end : bool } , }
/* FP:ty.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0018
/* FP:ty.rs-0036 */ # [doc = " Represents a constant in the type system"] # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct TyConst { pub (crate) kind : TyConstKind , pub id : TyConstId , }
/* FP:ty.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0019
/* FP:ty.rs-0038 */ impl TyConst { pub fn new (kind : TyConstKind , id : TyConstId) -> TyConst { Self { kind , id } } # [doc = " Retrieve the constant kind."] pub fn kind (& self) -> & TyConstKind { & self . kind } # [doc = " Creates an interned usize constant."] pub fn try_from_target_usize (val : u64) -> Result < Self , Error > { with (| cx | cx . try_new_ty_const_uint (val . into () , UintTy :: Usize)) } # [doc = " Try to evaluate to a target `usize`."] pub fn eval_target_usize (& self) -> Result < u64 , Error > { with (| cx | cx . eval_target_usize_ty (self)) } }
/* FP:ty.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0020
/* FP:ty.rs-0040 */ # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum TyConstKind { Param (ParamConst) , Bound (DebruijnIndex , BoundVar) , Unevaluated (ConstDef , GenericArgs) , Value (Ty , Allocation) , ZSTValue (Ty) , }
/* FP:ty.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0021
/* FP:ty.rs-0042 */ # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct TyConstId (usize) ;
/* FP:ty.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0022
/* FP:ty.rs-0044 */ # [doc = " Represents a constant in MIR"] # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct MirConst { # [doc = " The constant kind."] pub (crate) kind : ConstantKind , # [doc = " The constant type."] pub (crate) ty : Ty , # [doc = " Used for internal tracking of the internal constant."] pub id : MirConstId , }
/* FP:ty.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0023
/* FP:ty.rs-0046 */ impl MirConst { # [doc = " Build a constant. Note that this should only be used by the compiler."] pub fn new (kind : ConstantKind , ty : Ty , id : MirConstId) -> MirConst { MirConst { kind , ty , id } } # [doc = " Retrieve the constant kind."] pub fn kind (& self) -> & ConstantKind { & self . kind } # [doc = " Get the constant type."] pub fn ty (& self) -> Ty { self . ty } # [doc = " Try to evaluate to a target `usize`."] pub fn eval_target_usize (& self) -> Result < u64 , Error > { with (| cx | cx . eval_target_usize (self)) } # [doc = " Create a constant that represents a new zero-sized constant of type T."] # [doc = " Fails if the type is not a ZST or if it doesn't have a known size."] pub fn try_new_zero_sized (ty : Ty) -> Result < MirConst , Error > { with (| cx | cx . try_new_const_zst (ty)) } # [doc = " Build a new constant that represents the given string."] # [doc = ""] # [doc = " Note that there is no guarantee today about duplication of the same constant."] # [doc = " I.e.: Calling this function multiple times with the same argument may or may not return"] # [doc = " the same allocation."] pub fn from_str (value : & str) -> MirConst { with (| cx | cx . new_const_str (value)) } # [doc = " Build a new constant that represents the given boolean value."] pub fn from_bool (value : bool) -> MirConst { with (| cx | cx . new_const_bool (value)) } # [doc = " Build a new constant that represents the given unsigned integer."] pub fn try_from_uint (value : u128 , uint_ty : UintTy) -> Result < MirConst , Error > { with (| cx | cx . try_new_const_uint (value , uint_ty)) } }
/* FP:ty.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0024
/* FP:ty.rs-0048 */ # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , Serialize)] pub struct MirConstId (usize) ;
/* FP:ty.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_TYPE_0025
/* FP:ty.rs-0050 */ type Ident = Opaque ;
/* FP:ty.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0026
/* FP:ty.rs-0052 */ # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct Region { pub kind : RegionKind , }
/* FP:ty.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0027
/* FP:ty.rs-0054 */ # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum RegionKind { ReEarlyParam (EarlyParamRegion) , ReBound (DebruijnIndex , BoundRegion) , ReStatic , RePlaceholder (Placeholder < BoundRegion >) , ReErased , }
/* FP:ty.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_TYPE_0028
/* FP:ty.rs-0056 */ pub (crate) type DebruijnIndex = u32 ;
/* FP:ty.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0029
/* FP:ty.rs-0058 */ # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct EarlyParamRegion { pub index : u32 , pub name : Symbol , }
/* FP:ty.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_TYPE_0030
/* FP:ty.rs-0060 */ pub (crate) type BoundVar = u32 ;
/* FP:ty.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0031
/* FP:ty.rs-0062 */ # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct BoundRegion { pub var : BoundVar , pub kind : BoundRegionKind , }
/* FP:ty.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_TYPE_0032
/* FP:ty.rs-0064 */ pub (crate) type UniverseIndex = u32 ;
/* FP:ty.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0033
/* FP:ty.rs-0066 */ # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct Placeholder < T > { pub universe : UniverseIndex , pub bound : T , }
/* FP:ty.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0034
/* FP:ty.rs-0068 */ # [derive (Clone , Copy , PartialEq , Eq , Hash , Serialize)] pub struct Span (usize) ;
/* FP:ty.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0035
/* FP:ty.rs-0070 */ impl Debug for Span { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Span") . field ("id" , & self . 0) . field ("repr" , & with (| cx | cx . span_to_string (* self))) . finish () } }
/* FP:ty.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0036
/* FP:ty.rs-0072 */ impl Span { # [doc = " Return filename for diagnostic purposes"] pub fn get_filename (& self) -> Filename { with (| c | c . get_filename (self)) } # [doc = " Return lines that correspond to this `Span`"] pub fn get_lines (& self) -> LineInfo { with (| c | c . get_lines (self)) } # [doc = " Return the span location to be printed in diagnostic messages."] # [doc = ""] # [doc = " This may leak local file paths and should not be used to build artifacts that may be"] # [doc = " distributed."] pub fn diagnostic (& self) -> String { with (| c | c . span_to_string (* self)) } }
/* FP:ty.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0037
/* FP:ty.rs-0074 */ # [derive (Clone , Copy , Debug , Serialize)] # [doc = " Information you get from `Span` in a struct form."] # [doc = " Line and col start from 1."] pub struct LineInfo { pub start_line : usize , pub start_col : usize , pub end_line : usize , pub end_col : usize , }
/* FP:ty.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0038
/* FP:ty.rs-0076 */ impl LineInfo { pub fn from (lines : (usize , usize , usize , usize)) -> Self { LineInfo { start_line : lines . 0 , start_col : lines . 1 , end_line : lines . 2 , end_col : lines . 3 } } }
/* FP:ty.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0039
/* FP:ty.rs-0078 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum TyKind { RigidTy (RigidTy) , Alias (AliasKind , AliasTy) , Param (ParamTy) , Bound (usize , BoundTy) , }
/* FP:ty.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0040
/* FP:ty.rs-0080 */ impl TyKind { pub fn rigid (& self) -> Option < & RigidTy > { if let TyKind :: RigidTy (inner) = self { Some (inner) } else { None } } # [inline] pub fn is_unit (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Tuple (data)) if data . is_empty ()) } # [inline] pub fn is_bool (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Bool)) } # [inline] pub fn is_char (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Char)) } # [inline] pub fn is_trait (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Dynamic (_ , _ , DynKind :: Dyn))) } # [inline] pub fn is_enum (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Adt (def , _)) if def . kind () == AdtKind :: Enum) } # [inline] pub fn is_struct (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Adt (def , _)) if def . kind () == AdtKind :: Struct) } # [inline] pub fn is_union (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Adt (def , _)) if def . kind () == AdtKind :: Union) } # [inline] pub fn is_adt (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Adt (..))) } # [inline] pub fn is_ref (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Ref (..))) } # [inline] pub fn is_fn (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: FnDef (..))) } # [inline] pub fn is_fn_ptr (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: FnPtr (..))) } # [inline] pub fn is_primitive (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Bool | RigidTy :: Char | RigidTy :: Int (_) | RigidTy :: Uint (_) | RigidTy :: Float (_))) } # [inline] pub fn is_float (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Float (_))) } # [inline] pub fn is_integral (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Int (_) | RigidTy :: Uint (_))) } # [inline] pub fn is_numeric (& self) -> bool { self . is_integral () || self . is_float () } # [inline] pub fn is_signed (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Int (_))) } # [inline] pub fn is_str (& self) -> bool { * self == TyKind :: RigidTy (RigidTy :: Str) } # [inline] pub fn is_cstr (& self) -> bool { let TyKind :: RigidTy (RigidTy :: Adt (def , _)) = self else { return false ; } ; with (| cx | cx . adt_is_cstr (* def)) } # [inline] pub fn is_slice (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Slice (_))) } # [inline] pub fn is_array (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Array (..))) } # [inline] pub fn is_mutable_ptr (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: RawPtr (_ , Mutability :: Mut)) | TyKind :: RigidTy (RigidTy :: Ref (_ , _ , Mutability :: Mut))) } # [inline] pub fn is_raw_ptr (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: RawPtr (..))) } # [doc = " Tests if this is any kind of primitive pointer type (reference, raw pointer, fn pointer)."] # [inline] pub fn is_any_ptr (& self) -> bool { self . is_ref () || self . is_raw_ptr () || self . is_fn_ptr () } # [inline] pub fn is_coroutine (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Coroutine (..))) } # [inline] pub fn is_closure (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Closure (..))) } # [inline] pub fn is_box (& self) -> bool { match self { TyKind :: RigidTy (RigidTy :: Adt (def , _)) => def . is_box () , _ => false , } } # [inline] pub fn is_simd (& self) -> bool { matches ! (self , TyKind :: RigidTy (RigidTy :: Adt (def , _)) if def . is_simd ()) } pub fn trait_principal (& self) -> Option < Binder < ExistentialTraitRef > > { if let TyKind :: RigidTy (RigidTy :: Dynamic (predicates , _ , _)) = self { if let Some (Binder { value : ExistentialPredicate :: Trait (trait_ref) , bound_vars }) = predicates . first () { Some (Binder { value : trait_ref . clone () , bound_vars : bound_vars . clone () }) } else { None } } else { None } } # [doc = " Returns the type of `ty[i]` for builtin types."] pub fn builtin_index (& self) -> Option < Ty > { match self . rigid () ? { RigidTy :: Array (ty , _) | RigidTy :: Slice (ty) => Some (* ty) , _ => None , } } # [doc = " Returns the type and mutability of `*ty` for builtin types."] # [doc = ""] # [doc = " The parameter `explicit` indicates if this is an *explicit* dereference."] # [doc = " Some types -- notably raw ptrs -- can only be dereferenced explicitly."] pub fn builtin_deref (& self , explicit : bool) -> Option < TypeAndMut > { match self . rigid () ? { RigidTy :: Adt (def , args) if def . is_box () => { Some (TypeAndMut { ty : * args . 0 . first () ? . ty () ? , mutability : Mutability :: Not }) } RigidTy :: Ref (_ , ty , mutability) => { Some (TypeAndMut { ty : * ty , mutability : * mutability }) } RigidTy :: RawPtr (ty , mutability) if explicit => { Some (TypeAndMut { ty : * ty , mutability : * mutability }) } _ => None , } } # [doc = " Get the function signature for function like types (Fn, FnPtr, and Closure)"] pub fn fn_sig (& self) -> Option < PolyFnSig > { match self { TyKind :: RigidTy (RigidTy :: FnDef (def , args)) => Some (with (| cx | cx . fn_sig (* def , args))) , TyKind :: RigidTy (RigidTy :: FnPtr (sig)) => Some (sig . clone ()) , TyKind :: RigidTy (RigidTy :: Closure (_def , args)) => Some (with (| cx | cx . closure_sig (args))) , _ => None , } } # [doc = " Get the discriminant type for this type."] pub fn discriminant_ty (& self) -> Option < Ty > { self . rigid () . map (| ty | with (| cx | cx . rigid_ty_discriminant_ty (ty))) } # [doc = " Deconstruct a function type if this is one."] pub fn fn_def (& self) -> Option < (FnDef , & GenericArgs) > { if let TyKind :: RigidTy (RigidTy :: FnDef (def , args)) = self { Some ((* def , args)) } else { None } } }
/* FP:ty.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0041
/* FP:ty.rs-0082 */ pub struct TypeAndMut { pub ty : Ty , pub mutability : Mutability , }
/* FP:ty.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0042
/* FP:ty.rs-0084 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum RigidTy { Bool , Char , Int (IntTy) , Uint (UintTy) , Float (FloatTy) , Adt (AdtDef , GenericArgs) , Foreign (ForeignDef) , Str , Array (Ty , TyConst) , Pat (Ty , Pattern) , Slice (Ty) , RawPtr (Ty , Mutability) , Ref (Region , Ty , Mutability) , FnDef (FnDef , GenericArgs) , FnPtr (PolyFnSig) , Closure (ClosureDef , GenericArgs) , Coroutine (CoroutineDef , GenericArgs) , CoroutineClosure (CoroutineClosureDef , GenericArgs) , Dynamic (Vec < Binder < ExistentialPredicate > > , Region , DynKind) , Never , Tuple (Vec < Ty >) , CoroutineWitness (CoroutineWitnessDef , GenericArgs) , }
/* FP:ty.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0043
/* FP:ty.rs-0086 */ impl RigidTy { # [doc = " Get the discriminant type for this type."] pub fn discriminant_ty (& self) -> Ty { with (| cx | cx . rigid_ty_discriminant_ty (self)) } }
/* FP:ty.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0044
/* FP:ty.rs-0088 */ impl From < RigidTy > for TyKind { fn from (value : RigidTy) -> Self { TyKind :: RigidTy (value) } }
/* FP:ty.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0045
/* FP:ty.rs-0090 */ # [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum IntTy { Isize , I8 , I16 , I32 , I64 , I128 , }
/* FP:ty.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0046
/* FP:ty.rs-0092 */ impl IntTy { pub fn num_bytes (self) -> usize { match self { IntTy :: Isize => MachineInfo :: target_pointer_width () . bytes () , IntTy :: I8 => 1 , IntTy :: I16 => 2 , IntTy :: I32 => 4 , IntTy :: I64 => 8 , IntTy :: I128 => 16 , } } }
/* FP:ty.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0047
/* FP:ty.rs-0094 */ # [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum UintTy { Usize , U8 , U16 , U32 , U64 , U128 , }
/* FP:ty.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0048
/* FP:ty.rs-0096 */ impl UintTy { pub fn num_bytes (self) -> usize { match self { UintTy :: Usize => MachineInfo :: target_pointer_width () . bytes () , UintTy :: U8 => 1 , UintTy :: U16 => 2 , UintTy :: U32 => 4 , UintTy :: U64 => 8 , UintTy :: U128 => 16 , } } }
/* FP:ty.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0049
/* FP:ty.rs-0098 */ # [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum FloatTy { F16 , F32 , F64 , F128 , }
/* FP:ty.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0050
/* FP:ty.rs-0100 */ # [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum Movability { Static , Movable , }
/* FP:ty.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0051
/* FP:ty.rs-0102 */ crate_def ! { # [derive (Serialize)] pub ForeignModuleDef ; }
/* FP:ty.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0052
/* FP:ty.rs-0104 */ impl ForeignModuleDef { pub fn module (& self) -> ForeignModule { with (| cx | cx . foreign_module (* self)) } }
/* FP:ty.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0053
/* FP:ty.rs-0106 */ pub struct ForeignModule { pub def_id : ForeignModuleDef , pub abi : Abi , }
/* FP:ty.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0054
/* FP:ty.rs-0108 */ impl ForeignModule { pub fn items (& self) -> Vec < ForeignDef > { with (| cx | cx . foreign_items (self . def_id)) } }
/* FP:ty.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0055
/* FP:ty.rs-0110 */ crate_def_with_ty ! { # [doc = " Hold information about a ForeignItem in a crate."] # [derive (Serialize)] pub ForeignDef ; }
/* FP:ty.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0056
/* FP:ty.rs-0112 */ impl ForeignDef { pub fn kind (& self) -> ForeignItemKind { with (| cx | cx . foreign_item_kind (* self)) } }
/* FP:ty.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0057
/* FP:ty.rs-0114 */ # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub enum ForeignItemKind { Fn (FnDef) , Static (StaticDef) , Type (Ty) , }
/* FP:ty.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0058
/* FP:ty.rs-0116 */ crate_def_with_ty ! { # [doc = " Hold information about a function definition in a crate."] # [derive (Serialize)] pub FnDef ; }
/* FP:ty.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0059
/* FP:ty.rs-0118 */ impl FnDef { pub fn body (& self) -> Option < Body > { with (| ctx | ctx . has_body (self . 0) . then (| | ctx . mir_body (self . 0))) } pub fn has_body (& self) -> bool { with (| ctx | ctx . has_body (self . 0)) } # [doc = " Get the information of the intrinsic if this function is a definition of one."] pub fn as_intrinsic (& self) -> Option < IntrinsicDef > { with (| cx | cx . intrinsic (self . def_id ())) } # [doc = " Check if the function is an intrinsic."] # [inline] pub fn is_intrinsic (& self) -> bool { self . as_intrinsic () . is_some () } # [doc = " Get the function signature for this function definition."] pub fn fn_sig (& self) -> PolyFnSig { let kind = self . ty () . kind () ; kind . fn_sig () . unwrap () } }
/* FP:ty.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0060
/* FP:ty.rs-0120 */ crate_def_with_ty ! { # [derive (Serialize)] pub IntrinsicDef ; }
/* FP:ty.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0061
/* FP:ty.rs-0122 */ impl IntrinsicDef { # [doc = " Returns the plain name of the intrinsic."] # [doc = " e.g., `transmute` for `core::intrinsics::transmute`."] pub fn fn_name (& self) -> Symbol { with (| cx | cx . intrinsic_name (* self)) } # [doc = " Returns whether the intrinsic has no meaningful body and all backends"] # [doc = " need to shim all calls to it."] pub fn must_be_overridden (& self) -> bool { with (| cx | ! cx . has_body (self . 0)) } }
/* FP:ty.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0062
/* FP:ty.rs-0124 */ impl From < IntrinsicDef > for FnDef { fn from (def : IntrinsicDef) -> Self { FnDef (def . 0) } }
/* FP:ty.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0063
/* FP:ty.rs-0126 */ crate_def ! { # [derive (Serialize)] pub ClosureDef ; }
/* FP:ty.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0064
/* FP:ty.rs-0128 */ impl ClosureDef { # [doc = " Retrieves the body of the closure definition. Returns None if the body"] # [doc = " isn't available."] pub fn body (& self) -> Option < Body > { with (| ctx | ctx . has_body (self . 0) . then (| | ctx . mir_body (self . 0))) } }
/* FP:ty.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0065
/* FP:ty.rs-0130 */ crate_def ! { # [derive (Serialize)] pub CoroutineDef ; }
/* FP:ty.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0066
/* FP:ty.rs-0132 */ impl CoroutineDef { # [doc = " Retrieves the body of the coroutine definition. Returns None if the body"] # [doc = " isn't available."] pub fn body (& self) -> Option < Body > { with (| cx | cx . has_body (self . 0) . then (| | cx . mir_body (self . 0))) } pub fn discriminant_for_variant (& self , args : & GenericArgs , idx : VariantIdx) -> Discr { with (| cx | cx . coroutine_discr_for_variant (* self , args , idx)) } }
/* FP:ty.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0067
/* FP:ty.rs-0134 */ crate_def ! { # [derive (Serialize)] pub CoroutineClosureDef ; }
/* FP:ty.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0068
/* FP:ty.rs-0136 */ crate_def ! { # [derive (Serialize)] pub ParamDef ; }
/* FP:ty.rs-0137 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0069
/* FP:ty.rs-0138 */ crate_def ! { # [derive (Serialize)] pub BrNamedDef ; }
/* FP:ty.rs-0139 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0070
/* FP:ty.rs-0140 */ crate_def ! { # [derive (Serialize)] pub AdtDef ; }
/* FP:ty.rs-0141 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0071
/* FP:ty.rs-0142 */ # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub enum AdtKind { Enum , Union , Struct , }
/* FP:ty.rs-0143 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0072
/* FP:ty.rs-0144 */ impl AdtDef { pub fn kind (& self) -> AdtKind { with (| cx | cx . adt_kind (* self)) } # [doc = " Retrieve the type of this Adt."] pub fn ty (& self) -> Ty { with (| cx | cx . def_ty (self . 0)) } # [doc = " Retrieve the type of this Adt by instantiating and normalizing it with the given arguments."] # [doc = ""] # [doc = " This will assume the type can be instantiated with these arguments."] pub fn ty_with_args (& self , args : & GenericArgs) -> Ty { with (| cx | cx . def_ty_with_args (self . 0 , args)) } pub fn is_box (& self) -> bool { with (| cx | cx . adt_is_box (* self)) } pub fn is_simd (& self) -> bool { with (| cx | cx . adt_is_simd (* self)) } # [doc = " The number of variants in this ADT."] pub fn num_variants (& self) -> usize { with (| cx | cx . adt_variants_len (* self)) } # [doc = " Retrieve the variants in this ADT."] pub fn variants (& self) -> Vec < VariantDef > { self . variants_iter () . collect () } # [doc = " Iterate over the variants in this ADT."] pub fn variants_iter (& self) -> impl Iterator < Item = VariantDef > { (0 .. self . num_variants ()) . map (| idx | VariantDef { idx : VariantIdx :: to_val (idx) , adt_def : * self }) } pub fn variant (& self , idx : VariantIdx) -> Option < VariantDef > { (idx . to_index () < self . num_variants ()) . then_some (VariantDef { idx , adt_def : * self }) } pub fn repr (& self) -> ReprOptions { with (| cx | cx . adt_repr (* self)) } pub fn discriminant_for_variant (& self , idx : VariantIdx) -> Discr { with (| cx | cx . adt_discr_for_variant (* self , idx)) } }
/* FP:ty.rs-0145 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0073
/* FP:ty.rs-0146 */ pub struct Discr { pub val : u128 , pub ty : Ty , }
/* FP:ty.rs-0147 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0074
/* FP:ty.rs-0148 */ # [doc = " Definition of a variant, which can be either a struct / union field or an enum variant."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , Serialize)] pub struct VariantDef { # [doc = " The variant index."] # [doc = ""] # [doc = " ## Warning"] # [doc = " Do not access this field directly!"] pub idx : VariantIdx , # [doc = " The data type where this variant comes from."] # [doc = " For now, we use this to retrieve information about the variant itself so we don't need to"] # [doc = " cache more information."] # [doc = ""] # [doc = " ## Warning"] # [doc = " Do not access this field directly!"] pub adt_def : AdtDef , }
/* FP:ty.rs-0149 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0075
/* FP:ty.rs-0150 */ impl VariantDef { pub fn name (& self) -> Symbol { with (| cx | cx . variant_name (* self)) } # [doc = " Retrieve all the fields in this variant."] pub fn fields (& self) -> Vec < FieldDef > { with (| cx | cx . variant_fields (* self)) } }
/* FP:ty.rs-0151 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0076
/* FP:ty.rs-0152 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct FieldDef { # [doc = " The field definition."] # [doc = ""] # [doc = " ## Warning"] # [doc = " Do not access this field directly! This is public for the compiler to have access to it."] pub def : DefId , # [doc = " The field name."] pub name : Symbol , }
/* FP:ty.rs-0153 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0077
/* FP:ty.rs-0154 */ impl FieldDef { # [doc = " Retrieve the type of this field instantiating and normalizing it with the given arguments."] # [doc = ""] # [doc = " This will assume the type can be instantiated with these arguments."] pub fn ty_with_args (& self , args : & GenericArgs) -> Ty { with (| cx | cx . def_ty_with_args (self . def , args)) } # [doc = " Retrieve the type of this field."] pub fn ty (& self) -> Ty { with (| cx | cx . def_ty (self . def)) } }
/* FP:ty.rs-0155 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0078
/* FP:ty.rs-0156 */ impl Display for AdtKind { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_str (match self { AdtKind :: Enum => "enum" , AdtKind :: Union => "union" , AdtKind :: Struct => "struct" , }) } }
/* FP:ty.rs-0157 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0079
/* FP:ty.rs-0158 */ impl AdtKind { pub fn is_enum (& self) -> bool { matches ! (self , AdtKind :: Enum) } pub fn is_struct (& self) -> bool { matches ! (self , AdtKind :: Struct) } pub fn is_union (& self) -> bool { matches ! (self , AdtKind :: Union) } }
/* FP:ty.rs-0159 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0080
/* FP:ty.rs-0160 */ crate_def ! { # [derive (Serialize)] pub AliasDef ; }
/* FP:ty.rs-0161 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0081
/* FP:ty.rs-0162 */ crate_def ! { # [doc = " A trait's definition."] # [derive (Serialize)] pub TraitDef ; }
/* FP:ty.rs-0163 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0082
/* FP:ty.rs-0164 */ impl_crate_def_items ! { TraitDef ; }
/* FP:ty.rs-0165 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0083
/* FP:ty.rs-0166 */ impl TraitDef { pub fn declaration (trait_def : & TraitDef) -> TraitDecl { with (| cx | cx . trait_decl (trait_def)) } }
/* FP:ty.rs-0167 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0084
/* FP:ty.rs-0168 */ crate_def ! { # [derive (Serialize)] pub GenericDef ; }
/* FP:ty.rs-0169 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0085
/* FP:ty.rs-0170 */ crate_def_with_ty ! { # [derive (Serialize)] pub ConstDef ; }
/* FP:ty.rs-0171 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0086
/* FP:ty.rs-0172 */ crate_def ! { # [doc = " A trait impl definition."] # [derive (Serialize)] pub ImplDef ; }
/* FP:ty.rs-0173 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0087
/* FP:ty.rs-0174 */ impl_crate_def_items ! { ImplDef ; }
/* FP:ty.rs-0175 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0088
/* FP:ty.rs-0176 */ impl ImplDef { # [doc = " Retrieve information about this implementation."] pub fn trait_impl (& self) -> ImplTrait { with (| cx | cx . trait_impl (self)) } }
/* FP:ty.rs-0177 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0089
/* FP:ty.rs-0178 */ crate_def ! { # [derive (Serialize)] pub RegionDef ; }
/* FP:ty.rs-0179 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0090
/* FP:ty.rs-0180 */ crate_def ! { # [derive (Serialize)] pub CoroutineWitnessDef ; }
/* FP:ty.rs-0181 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0091
/* FP:ty.rs-0182 */ # [doc = " A list of generic arguments."] # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct GenericArgs (pub Vec < GenericArgKind >) ;
/* FP:ty.rs-0183 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0092
/* FP:ty.rs-0184 */ impl std :: ops :: Index < ParamTy > for GenericArgs { type Output = Ty ; fn index (& self , index : ParamTy) -> & Self :: Output { self . 0 [index . index as usize] . expect_ty () } }
/* FP:ty.rs-0185 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0093
/* FP:ty.rs-0186 */ impl std :: ops :: Index < ParamConst > for GenericArgs { type Output = TyConst ; fn index (& self , index : ParamConst) -> & Self :: Output { self . 0 [index . index as usize] . expect_const () } }
/* FP:ty.rs-0187 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0094
/* FP:ty.rs-0188 */ # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum GenericArgKind { Lifetime (Region) , Type (Ty) , Const (TyConst) , }
/* FP:ty.rs-0189 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0095
/* FP:ty.rs-0190 */ impl GenericArgKind { # [doc = " Panic if this generic argument is not a type, otherwise"] # [doc = " return the type."] # [track_caller] pub fn expect_ty (& self) -> & Ty { match self { GenericArgKind :: Type (ty) => ty , _ => panic ! ("{self:?}") , } } # [doc = " Panic if this generic argument is not a const, otherwise"] # [doc = " return the const."] # [track_caller] pub fn expect_const (& self) -> & TyConst { match self { GenericArgKind :: Const (c) => c , _ => panic ! ("{self:?}") , } } # [doc = " Return the generic argument type if applicable, otherwise return `None`."] pub fn ty (& self) -> Option < & Ty > { match self { GenericArgKind :: Type (ty) => Some (ty) , _ => None , } } }
/* FP:ty.rs-0191 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0096
/* FP:ty.rs-0192 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum TermKind { Type (Ty) , Const (TyConst) , }
/* FP:ty.rs-0193 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0097
/* FP:ty.rs-0194 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AliasKind { Projection , Inherent , Opaque , Free , }
/* FP:ty.rs-0195 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0098
/* FP:ty.rs-0196 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct AliasTy { pub def_id : AliasDef , pub args : GenericArgs , }
/* FP:ty.rs-0197 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0099
/* FP:ty.rs-0198 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct AliasTerm { pub def_id : AliasDef , pub args : GenericArgs , }
/* FP:ty.rs-0199 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_TYPE_0100
/* FP:ty.rs-0200 */ pub type PolyFnSig = Binder < FnSig > ;
/* FP:ty.rs-0201 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0101
/* FP:ty.rs-0202 */ impl PolyFnSig { # [doc = " Compute a `FnAbi` suitable for indirect calls, i.e. to `fn` pointers."] # [doc = ""] # [doc = " NB: this doesn't handle virtual calls - those should use `Instance::fn_abi`"] # [doc = " instead, where the instance is an `InstanceKind::Virtual`."] pub fn fn_ptr_abi (self) -> Result < FnAbi , Error > { with (| cx | cx . fn_ptr_abi (self)) } }
/* FP:ty.rs-0203 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0102
/* FP:ty.rs-0204 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct FnSig { pub inputs_and_output : Vec < Ty > , pub c_variadic : bool , pub safety : Safety , pub abi : Abi , }
/* FP:ty.rs-0205 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0103
/* FP:ty.rs-0206 */ impl FnSig { pub fn output (& self) -> Ty { self . inputs_and_output [self . inputs_and_output . len () - 1] } pub fn inputs (& self) -> & [Ty] { & self . inputs_and_output [.. self . inputs_and_output . len () - 1] } }
/* FP:ty.rs-0207 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0104
/* FP:ty.rs-0208 */ # [derive (Clone , PartialEq , Eq , Debug , Serialize)] pub enum Abi { Rust , C { unwind : bool } , Cdecl { unwind : bool } , Stdcall { unwind : bool } , Fastcall { unwind : bool } , Vectorcall { unwind : bool } , Thiscall { unwind : bool } , Aapcs { unwind : bool } , Win64 { unwind : bool } , SysV64 { unwind : bool } , PtxKernel , Msp430Interrupt , X86Interrupt , GpuKernel , EfiApi , AvrInterrupt , AvrNonBlockingInterrupt , CCmseNonSecureCall , CCmseNonSecureEntry , System { unwind : bool } , RustCall , Unadjusted , RustCold , RiscvInterruptM , RiscvInterruptS , RustInvalid , Custom , }
/* FP:ty.rs-0209 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0105
/* FP:ty.rs-0210 */ # [doc = " A binder represents a possibly generic type and its bound vars."] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct Binder < T > { pub value : T , pub bound_vars : Vec < BoundVariableKind > , }
/* FP:ty.rs-0211 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0106
/* FP:ty.rs-0212 */ impl < T > Binder < T > { # [doc = " Create a new binder with the given bound vars."] pub fn bind_with_vars (value : T , bound_vars : Vec < BoundVariableKind >) -> Self { Binder { value , bound_vars } } # [doc = " Create a new binder with no bounded variable."] pub fn dummy (value : T) -> Self { Binder { value , bound_vars : vec ! [] } } pub fn skip_binder (self) -> T { self . value } pub fn map_bound_ref < F , U > (& self , f : F) -> Binder < U > where F : FnOnce (& T) -> U , { let Binder { value , bound_vars } = self ; let new_value = f (value) ; Binder { value : new_value , bound_vars : bound_vars . clone () } } pub fn map_bound < F , U > (self , f : F) -> Binder < U > where F : FnOnce (T) -> U , { let Binder { value , bound_vars } = self ; let new_value = f (value) ; Binder { value : new_value , bound_vars } } }
/* FP:ty.rs-0213 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0107
/* FP:ty.rs-0214 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct EarlyBinder < T > { pub value : T , }
/* FP:ty.rs-0215 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0108
/* FP:ty.rs-0216 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum BoundVariableKind { Ty (BoundTyKind) , Region (BoundRegionKind) , Const , }
/* FP:ty.rs-0217 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0109
/* FP:ty.rs-0218 */ # [derive (Clone , PartialEq , Eq , Debug , Serialize)] pub enum BoundTyKind { Anon , Param (ParamDef , String) , }
/* FP:ty.rs-0219 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0110
/* FP:ty.rs-0220 */ # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum BoundRegionKind { BrAnon , BrNamed (BrNamedDef , String) , BrEnv , }
/* FP:ty.rs-0221 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0111
/* FP:ty.rs-0222 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum DynKind { Dyn , }
/* FP:ty.rs-0223 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0112
/* FP:ty.rs-0224 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ExistentialPredicate { Trait (ExistentialTraitRef) , Projection (ExistentialProjection) , AutoTrait (TraitDef) , }
/* FP:ty.rs-0225 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0113
/* FP:ty.rs-0226 */ # [doc = " An existential reference to a trait where `Self` is not included."] # [doc = ""] # [doc = " The `generic_args` will include any other known argument."] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct ExistentialTraitRef { pub def_id : TraitDef , pub generic_args : GenericArgs , }
/* FP:ty.rs-0227 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0114
/* FP:ty.rs-0228 */ impl Binder < ExistentialTraitRef > { pub fn with_self_ty (& self , self_ty : Ty) -> Binder < TraitRef > { self . map_bound_ref (| trait_ref | trait_ref . with_self_ty (self_ty)) } }
/* FP:ty.rs-0229 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0115
/* FP:ty.rs-0230 */ impl ExistentialTraitRef { pub fn with_self_ty (& self , self_ty : Ty) -> TraitRef { TraitRef :: new (self . def_id , self_ty , & self . generic_args) } }
/* FP:ty.rs-0231 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0116
/* FP:ty.rs-0232 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct ExistentialProjection { pub def_id : TraitDef , pub generic_args : GenericArgs , pub term : TermKind , }
/* FP:ty.rs-0233 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0117
/* FP:ty.rs-0234 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct ParamTy { pub index : u32 , pub name : String , }
/* FP:ty.rs-0235 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0118
/* FP:ty.rs-0236 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct BoundTy { pub var : usize , pub kind : BoundTyKind , }
/* FP:ty.rs-0237 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_TYPE_0119
/* FP:ty.rs-0238 */ pub type Bytes = Vec < Option < u8 > > ;
/* FP:ty.rs-0239 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_TYPE_0120
/* FP:ty.rs-0240 */ # [doc = " Size in bytes."] pub type Size = usize ;
/* FP:ty.rs-0241 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0121
/* FP:ty.rs-0242 */ # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub struct Prov (pub AllocId) ;
/* FP:ty.rs-0243 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_TYPE_0122
/* FP:ty.rs-0244 */ pub type Align = u64 ;
/* FP:ty.rs-0245 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_TYPE_0123
/* FP:ty.rs-0246 */ pub type Promoted = u32 ;
/* FP:ty.rs-0247 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_TYPE_0124
/* FP:ty.rs-0248 */ pub type InitMaskMaterialized = Vec < u64 > ;
/* FP:ty.rs-0249 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0125
/* FP:ty.rs-0250 */ # [doc = " Stores the provenance information of pointers stored in memory."] # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct ProvenanceMap { # [doc = " Provenance in this map applies from the given offset for an entire pointer-size worth of"] # [doc = " bytes. Two entries in this map are always at least a pointer size apart."] pub ptrs : Vec < (Size , Prov) > , }
/* FP:ty.rs-0251 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0126
/* FP:ty.rs-0252 */ # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct Allocation { pub bytes : Bytes , pub provenance : ProvenanceMap , pub align : Align , pub mutability : Mutability , }
/* FP:ty.rs-0253 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0127
/* FP:ty.rs-0254 */ impl Allocation { # [doc = " Get a vector of bytes for an Allocation that has been fully initialized"] pub fn raw_bytes (& self) -> Result < Vec < u8 > , Error > { self . bytes . iter () . copied () . collect :: < Option < Vec < _ > > > () . ok_or_else (| | error ! ("Found uninitialized bytes: `{:?}`" , self . bytes)) } # [doc = " Read a uint value from the specified range."] pub fn read_partial_uint (& self , range : Range < usize >) -> Result < u128 , Error > { if range . end - range . start > 16 { return Err (error ! ("Allocation is bigger than largest integer")) ; } if range . end > self . bytes . len () { return Err (error ! ("Range is out of bounds. Allocation length is `{}`, but requested range `{:?}`" , self . bytes . len () , range)) ; } let raw = self . bytes [range] . iter () . copied () . collect :: < Option < Vec < _ > > > () . ok_or_else (| | error ! ("Found uninitialized bytes: `{:?}`" , self . bytes)) ? ; read_target_uint (& raw) } # [doc = " Read this allocation and try to convert it to an unassigned integer."] pub fn read_uint (& self) -> Result < u128 , Error > { if self . bytes . len () > 16 { return Err (error ! ("Allocation is bigger than largest integer")) ; } let raw = self . raw_bytes () ? ; read_target_uint (& raw) } # [doc = " Read this allocation and try to convert it to a signed integer."] pub fn read_int (& self) -> Result < i128 , Error > { if self . bytes . len () > 16 { return Err (error ! ("Allocation is bigger than largest integer")) ; } let raw = self . raw_bytes () ? ; read_target_int (& raw) } # [doc = " Read this allocation and try to convert it to a boolean."] pub fn read_bool (& self) -> Result < bool , Error > { match self . read_int () ? { 0 => Ok (false) , 1 => Ok (true) , val => Err (error ! ("Unexpected value for bool: `{val}`")) , } } # [doc = " Read this allocation as a pointer and return whether it represents a `null` pointer."] pub fn is_null (& self) -> Result < bool , Error > { let len = self . bytes . len () ; let ptr_len = MachineInfo :: target_pointer_width () . bytes () ; if len != ptr_len { return Err (error ! ("Expected width of pointer (`{ptr_len}`), but found: `{len}`")) ; } Ok (self . read_uint () ? == 0 && self . provenance . ptrs . is_empty ()) } }
/* FP:ty.rs-0255 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0128
/* FP:ty.rs-0256 */ # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum ConstantKind { Ty (TyConst) , Allocated (Allocation) , Unevaluated (UnevaluatedConst) , Param (ParamConst) , # [doc = " Store ZST constants."] # [doc = " We have to special handle these constants since its type might be generic."] ZeroSized , }
/* FP:ty.rs-0257 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0129
/* FP:ty.rs-0258 */ # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct ParamConst { pub index : u32 , pub name : String , }
/* FP:ty.rs-0259 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0130
/* FP:ty.rs-0260 */ # [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub struct UnevaluatedConst { pub def : ConstDef , pub args : GenericArgs , pub promoted : Option < Promoted > , }
/* FP:ty.rs-0261 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0131
/* FP:ty.rs-0262 */ # [derive (Clone , Copy , Debug , PartialEq , Eq , Serialize)] pub enum TraitSpecializationKind { None , Marker , AlwaysApplicable , }
/* FP:ty.rs-0263 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0132
/* FP:ty.rs-0264 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct TraitDecl { pub def_id : TraitDef , pub safety : Safety , pub paren_sugar : bool , pub has_auto_impl : bool , pub is_marker : bool , pub is_coinductive : bool , pub skip_array_during_method_dispatch : bool , pub skip_boxed_slice_during_method_dispatch : bool , pub specialization_kind : TraitSpecializationKind , pub must_implement_one_of : Option < Vec < Ident > > , pub implement_via_object : bool , pub deny_explicit_impl : bool , }
/* FP:ty.rs-0265 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0133
/* FP:ty.rs-0266 */ impl TraitDecl { pub fn generics_of (& self) -> Generics { with (| cx | cx . generics_of (self . def_id . 0)) } pub fn predicates_of (& self) -> GenericPredicates { with (| cx | cx . predicates_of (self . def_id . 0)) } pub fn explicit_predicates_of (& self) -> GenericPredicates { with (| cx | cx . explicit_predicates_of (self . def_id . 0)) } }
/* FP:ty.rs-0267 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_TYPE_0134
/* FP:ty.rs-0268 */ pub type ImplTrait = EarlyBinder < TraitRef > ;
/* FP:ty.rs-0269 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0135
/* FP:ty.rs-0270 */ # [doc = " A complete reference to a trait, i.e., one where `Self` is known."] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct TraitRef { pub def_id : TraitDef , # [doc = " The generic arguments for this definition."] # [doc = " The first element must always be type, and it represents `Self`."] args : GenericArgs , }
/* FP:ty.rs-0271 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0136
/* FP:ty.rs-0272 */ impl TraitRef { pub fn new (def_id : TraitDef , self_ty : Ty , gen_args : & GenericArgs) -> TraitRef { let mut args = vec ! [GenericArgKind :: Type (self_ty)] ; args . extend_from_slice (& gen_args . 0) ; TraitRef { def_id , args : GenericArgs (args) } } pub fn try_new (def_id : TraitDef , args : GenericArgs) -> Result < TraitRef , () > { match & args . 0 [..] { [GenericArgKind :: Type (_) , ..] => Ok (TraitRef { def_id , args }) , _ => Err (()) , } } pub fn args (& self) -> & GenericArgs { & self . args } pub fn self_ty (& self) -> Ty { let GenericArgKind :: Type (self_ty) = self . args . 0 [0] else { panic ! ("Self must be a type, but found: {:?}" , self . args . 0 [0]) } ; self_ty } }
/* FP:ty.rs-0273 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0137
/* FP:ty.rs-0274 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct Generics { pub parent : Option < GenericDef > , pub parent_count : usize , pub params : Vec < GenericParamDef > , pub param_def_id_to_index : Vec < (GenericDef , u32) > , pub has_self : bool , pub has_late_bound_regions : Option < Span > , }
/* FP:ty.rs-0275 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0138
/* FP:ty.rs-0276 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum GenericParamDefKind { Lifetime , Type { has_default : bool , synthetic : bool } , Const { has_default : bool } , }
/* FP:ty.rs-0277 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0139
/* FP:ty.rs-0278 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct GenericParamDef { pub name : super :: Symbol , pub def_id : GenericDef , pub index : u32 , pub pure_wrt_drop : bool , pub kind : GenericParamDefKind , }
/* FP:ty.rs-0279 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0140
/* FP:ty.rs-0280 */ pub struct GenericPredicates { pub parent : Option < TraitDef > , pub predicates : Vec < (PredicateKind , Span) > , }
/* FP:ty.rs-0281 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0141
/* FP:ty.rs-0282 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum PredicateKind { Clause (ClauseKind) , DynCompatible (TraitDef) , SubType (SubtypePredicate) , Coerce (CoercePredicate) , ConstEquate (TyConst , TyConst) , Ambiguous , AliasRelate (TermKind , TermKind , AliasRelationDirection) , }
/* FP:ty.rs-0283 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0142
/* FP:ty.rs-0284 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ClauseKind { Trait (TraitPredicate) , RegionOutlives (RegionOutlivesPredicate) , TypeOutlives (TypeOutlivesPredicate) , Projection (ProjectionPredicate) , ConstArgHasType (TyConst , Ty) , WellFormed (TermKind) , ConstEvaluatable (TyConst) , }
/* FP:ty.rs-0285 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0143
/* FP:ty.rs-0286 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ClosureKind { Fn , FnMut , FnOnce , }
/* FP:ty.rs-0287 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0144
/* FP:ty.rs-0288 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct SubtypePredicate { pub a : Ty , pub b : Ty , }
/* FP:ty.rs-0289 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0145
/* FP:ty.rs-0290 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct CoercePredicate { pub a : Ty , pub b : Ty , }
/* FP:ty.rs-0291 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0146
/* FP:ty.rs-0292 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AliasRelationDirection { Equate , Subtype , }
/* FP:ty.rs-0293 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0147
/* FP:ty.rs-0294 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct TraitPredicate { pub trait_ref : TraitRef , pub polarity : PredicatePolarity , }
/* FP:ty.rs-0295 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0148
/* FP:ty.rs-0296 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct OutlivesPredicate < A , B > (pub A , pub B) ;
/* FP:ty.rs-0297 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_TYPE_0149
/* FP:ty.rs-0298 */ pub type RegionOutlivesPredicate = OutlivesPredicate < Region , Region > ;
/* FP:ty.rs-0299 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_TYPE_0150
/* FP:ty.rs-0300 */ pub type TypeOutlivesPredicate = OutlivesPredicate < Ty , Region > ;
/* FP:ty.rs-0301 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0151
/* FP:ty.rs-0302 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct ProjectionPredicate { pub projection_term : AliasTerm , pub term : TermKind , }
/* FP:ty.rs-0303 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0152
/* FP:ty.rs-0304 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ImplPolarity { Positive , Negative , Reservation , }
/* FP:ty.rs-0305 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0153
/* FP:ty.rs-0306 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum PredicatePolarity { Positive , Negative , }
/* FP:ty.rs-0307 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0154
/* FP:ty.rs-0308 */ macro_rules ! index_impl { ($ name : ident) => { impl crate :: IndexedVal for $ name { fn to_val (index : usize) -> Self { $ name (index) } fn to_index (& self) -> usize { self . 0 } } } ; }
/* FP:ty.rs-0309 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0155
/* FP:ty.rs-0310 */ index_impl ! (TyConstId) ;
/* FP:ty.rs-0311 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0156
/* FP:ty.rs-0312 */ index_impl ! (MirConstId) ;
/* FP:ty.rs-0313 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0157
/* FP:ty.rs-0314 */ index_impl ! (Ty) ;
/* FP:ty.rs-0315 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0158
/* FP:ty.rs-0316 */ index_impl ! (Span) ;
/* FP:ty.rs-0317 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0159
/* FP:ty.rs-0318 */ # [doc = " The source-order index of a variant in a type."] # [doc = ""] # [doc = " For example, in the following types,"] # [doc = " ```ignore(illustrative)"] # [doc = " enum Demo1 {"] # [doc = "    Variant0 { a: bool, b: i32 },"] # [doc = "    Variant1 { c: u8, d: u64 },"] # [doc = " }"] # [doc = " struct Demo2 { e: u8, f: u16, g: u8 }"] # [doc = " ```"] # [doc = " `a` is in the variant with the `VariantIdx` of `0`,"] # [doc = " `c` is in the variant with the `VariantIdx` of `1`, and"] # [doc = " `g` is in the variant with the `VariantIdx` of `0`."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , Serialize)] pub struct VariantIdx (usize) ;
/* FP:ty.rs-0319 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0160
/* FP:ty.rs-0320 */ index_impl ! (VariantIdx) ;
/* FP:ty.rs-0321 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0161
/* FP:ty.rs-0322 */ crate_def ! { # [doc = " Hold information about an Opaque definition, particularly useful in `RPITIT`."] # [derive (Serialize)] pub OpaqueDef ; }
/* FP:ty.rs-0323 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_MACRO_0162
/* FP:ty.rs-0324 */ crate_def ! { # [derive (Serialize)] pub AssocDef ; }
/* FP:ty.rs-0325 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_STRUCT_0163
/* FP:ty.rs-0326 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct AssocItem { pub def_id : AssocDef , pub kind : AssocKind , pub container : AssocContainer , }
/* FP:ty.rs-0327 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0164
/* FP:ty.rs-0328 */ # [derive (Clone , PartialEq , Debug , Eq , Serialize)] pub enum AssocTypeData { Normal (Symbol) , # [doc = " The associated type comes from an RPITIT. It has no name, and the"] # [doc = " `ImplTraitInTraitData` provides additional information about its"] # [doc = " source."] Rpitit (ImplTraitInTraitData) , }
/* FP:ty.rs-0329 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0165
/* FP:ty.rs-0330 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AssocKind { Const { name : Symbol } , Fn { name : Symbol , has_self : bool } , Type { data : AssocTypeData } , }
/* FP:ty.rs-0331 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0166
/* FP:ty.rs-0332 */ # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AssocContainer { InherentImpl , # [doc = " The `AssocDef` points to the trait item being implemented."] TraitImpl (AssocDef) , Trait , }
/* FP:ty.rs-0333 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_ENUM_0167
/* FP:ty.rs-0334 */ # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash , Serialize)] pub enum ImplTraitInTraitData { Trait { fn_def_id : FnDef , opaque_def_id : OpaqueDef } , Impl { fn_def_id : FnDef } , }
/* FP:ty.rs-0335 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_ty_IMPL_0168
/* FP:ty.rs-0336 */ impl AssocItem { pub fn is_impl_trait_in_trait (& self) -> bool { matches ! (self . kind , AssocKind :: Type { data : AssocTypeData :: Rpitit (_) }) } }