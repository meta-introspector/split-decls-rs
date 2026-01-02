mkmod!{error, { 
                getname!(error);
                getsrc!(error);
                getpath!(error);
                get_deps!(error);
                get_crates!(error);
                mkinclude!(error);
                 
            }}
mkmod!{allocation, { 
                getname!(allocation);
                getsrc!(allocation);
                getpath!(allocation);
                get_deps!(allocation);
                get_crates!(allocation);
                mkinclude!(allocation);
                 
            }}
mkmod!{pointer, { 
                getname!(pointer);
                getsrc!(pointer);
                getpath!(pointer);
                get_deps!(pointer);
                get_crates!(pointer);
                mkinclude!(pointer);
                 
            }}
mkmod!{queries, { 
                getname!(queries);
                getsrc!(queries);
                getpath!(queries);
                get_deps!(queries);
                get_crates!(queries);
                mkinclude!(queries);
                 
            }}
mkmod!{value, { 
                getname!(value);
                getsrc!(value);
                getpath!(value);
                get_deps!(value);
                get_crates!(value);
                mkinclude!(value);
                 
            }}
mkuse!{use std :: io :: { Read , Write } ;}
mkuse!{use std :: num :: NonZero ;}
mkuse!{use std :: { fmt , io } ;}
mkuse!{use rustc_abi :: { AddressSpace , Align , Endian , HasDataLayout , Size } ;}
mkuse!{use rustc_ast :: { LitKind , Mutability } ;}
mkuse!{use rustc_data_structures :: fx :: FxHashMap ;}
mkuse!{use rustc_data_structures :: sharded :: ShardedHashMap ;}
mkuse!{use rustc_data_structures :: sync :: { AtomicU64 , Lock } ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_macros :: { HashStable , TyDecodable , TyEncodable , TypeFoldable , TypeVisitable } ;}
mkuse!{use rustc_serialize :: { Decodable , Encodable } ;}
mkuse!{use tracing :: { debug , trace } ;}
mkuse!{pub use { err_exhaust , err_inval , err_machine_stop , err_ub , err_ub_custom , err_ub_format , err_unsup , err_unsup_format , throw_exhaust , throw_inval , throw_machine_stop , throw_ub , throw_ub_custom , throw_ub_format , throw_unsup , throw_unsup_format , } ;}
mkuse!{pub use self :: allocation :: { AllocBytes , AllocError , AllocInit , AllocRange , AllocResult , Allocation , ConstAllocation , InitChunk , InitChunkIter , alloc_range , } ;}
mkuse!{pub use self :: error :: { BadBytesAccess , CheckAlignMsg , CheckInAllocMsg , ErrorHandled , EvalStaticInitializerRawResult , EvalToAllocationRawResult , EvalToConstValueResult , EvalToValTreeResult , ExpectedKind , InterpErrorInfo , InterpErrorKind , InterpResult , InvalidMetaKind , InvalidProgramInfo , MachineStopType , Misalignment , PointerKind , ReportedErrorInfo , ResourceExhaustionInfo , ScalarSizeMismatch , UndefinedBehaviorInfo , UnsupportedOpInfo , ValTreeCreationError , ValidationErrorInfo , ValidationErrorKind , interp_ok , } ;}
mkuse!{pub use self :: pointer :: { CtfeProvenance , Pointer , PointerArithmetic , Provenance } ;}
mkuse!{pub use self :: value :: Scalar ;}
mkuse!{use crate :: mir ;}
mkuse!{use crate :: ty :: codec :: { TyDecoder , TyEncoder } ;}
mkuse!{use crate :: ty :: print :: with_no_trimmed_paths ;}
mkuse!{use crate :: ty :: { self , Instance , Ty , TyCtxt } ;}
mkitem!{mkstruct!{# [doc = " Uniquely identifies one of the following:"] # [doc = " - A constant"] # [doc = " - A static"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , TyEncodable , TyDecodable)] # [derive (HashStable , TypeFoldable , TypeVisitable)] pub struct GlobalId < 'tcx > { # [doc = " For a constant or static, the `Instance` of the item itself."] # [doc = " For a promoted global, the `Instance` of the function they belong to."] pub instance : ty :: Instance < 'tcx > , # [doc = " The index for promoted globals within their function's `mir::Body`."] pub promoted : Option < mir :: Promoted > , }}}
mkitem!{mkimpl!{impl < 'tcx > GlobalId < 'tcx > { pub fn display (self , tcx : TyCtxt < 'tcx >) -> String { let instance_name = with_no_trimmed_paths ! (tcx . def_path_str (self . instance . def . def_id ())) ; if let Some (promoted) = self . promoted { format ! ("{instance_name}::{promoted:?}") } else { instance_name } } }}}
mkitem!{mkstruct!{# [doc = " Input argument for `tcx.lit_to_const`."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Hash , HashStable)] pub struct LitToConstInput < 'tcx > { # [doc = " The absolute value of the resultant constant."] pub lit : LitKind , # [doc = " The type of the constant."] pub ty : Ty < 'tcx > , # [doc = " If the constant is negative."] pub neg : bool , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Eq , Hash , Ord , PartialEq , PartialOrd)] pub struct AllocId (pub NonZero < u64 >) ;}}
mkitem!{mkimpl!{impl fmt :: Debug for AllocId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if f . alternate () { write ! (f , "a{}" , self . 0) } else { write ! (f , "alloc{}" , self . 0) } } }}}
mkitem!{mkenum!{# [derive (TyDecodable , TyEncodable)] enum AllocDiscriminant { Alloc , Fn , VTable , Static , Type , }}}

macro_rules! specialized_encode_alloc_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function specialized_encode_alloc_id in module {}", module_path!());
    };
}

mkfn!{
    specialized_encode_alloc_id_introspect!();
    pub fn specialized_encode_alloc_id < 'tcx , E : TyEncoder < 'tcx > > (encoder : & mut E , tcx : TyCtxt < 'tcx > , alloc_id : AllocId ,) { match tcx . global_alloc (alloc_id) { GlobalAlloc :: Memory (alloc) => { trace ! ("encoding {:?} with {:#?}" , alloc_id , alloc) ; AllocDiscriminant :: Alloc . encode (encoder) ; alloc . encode (encoder) ; } GlobalAlloc :: Function { instance } => { trace ! ("encoding {:?} with {:#?}" , alloc_id , instance) ; AllocDiscriminant :: Fn . encode (encoder) ; instance . encode (encoder) ; } GlobalAlloc :: VTable (ty , poly_trait_ref) => { trace ! ("encoding {:?} with {ty:#?}, {poly_trait_ref:#?}" , alloc_id) ; AllocDiscriminant :: VTable . encode (encoder) ; ty . encode (encoder) ; poly_trait_ref . encode (encoder) ; } GlobalAlloc :: TypeId { ty } => { trace ! ("encoding {alloc_id:?} with {ty:#?}") ; AllocDiscriminant :: Type . encode (encoder) ; ty . encode (encoder) ; } GlobalAlloc :: Static (did) => { assert ! (! tcx . is_thread_local_static (did)) ; AllocDiscriminant :: Static . encode (encoder) ; Encodable :: < E > :: encode (& did , encoder) ; } } }
}
mkitem!{mkenum!{# [derive (Clone)] enum State { Empty , Done (AllocId) , }}}
mkitem!{mkstruct!{pub struct AllocDecodingState { decoding_state : Vec < Lock < State > > , data_offsets : Vec < u64 > , }}}
mkitem!{mkimpl!{impl AllocDecodingState { # [inline] pub fn new_decoding_session (& self) -> AllocDecodingSession < '_ > { AllocDecodingSession { state : self } } pub fn new (data_offsets : Vec < u64 >) -> Self { let decoding_state = std :: iter :: repeat_with (| | Lock :: new (State :: Empty)) . take (data_offsets . len ()) . collect () ; Self { decoding_state , data_offsets } } }}}
mkitem!{mkstruct!{# [derive (Copy , Clone)] pub struct AllocDecodingSession < 's > { state : & 's AllocDecodingState , }}}
mkitem!{mkimpl!{impl < 's > AllocDecodingSession < 's > { # [doc = " Decodes an `AllocId` in a thread-safe way."] pub fn decode_alloc_id < 'tcx , D > (& self , decoder : & mut D) -> AllocId where D : TyDecoder < 'tcx > , { let idx = usize :: try_from (decoder . read_u32 ()) . unwrap () ; let pos = usize :: try_from (self . state . data_offsets [idx]) . unwrap () ; let (alloc_kind , pos) = decoder . with_position (pos , | decoder | { let alloc_kind = AllocDiscriminant :: decode (decoder) ; (alloc_kind , decoder . position ()) }) ; let mut entry = self . state . decoding_state [idx] . lock () ; if let State :: Done (alloc_id) = * entry { return alloc_id ; } let alloc_id = decoder . with_position (pos , | decoder | match alloc_kind { AllocDiscriminant :: Alloc => { trace ! ("creating memory alloc ID") ; let alloc = < ConstAllocation < 'tcx > as Decodable < _ > > :: decode (decoder) ; trace ! ("decoded alloc {:?}" , alloc) ; decoder . interner () . reserve_and_set_memory_alloc (alloc) } AllocDiscriminant :: Fn => { trace ! ("creating fn alloc ID") ; let instance = ty :: Instance :: decode (decoder) ; trace ! ("decoded fn alloc instance: {:?}" , instance) ; decoder . interner () . reserve_and_set_fn_alloc (instance , CTFE_ALLOC_SALT) } AllocDiscriminant :: VTable => { trace ! ("creating vtable alloc ID") ; let ty = Decodable :: decode (decoder) ; let poly_trait_ref = Decodable :: decode (decoder) ; trace ! ("decoded vtable alloc instance: {ty:?}, {poly_trait_ref:?}") ; decoder . interner () . reserve_and_set_vtable_alloc (ty , poly_trait_ref , CTFE_ALLOC_SALT) } AllocDiscriminant :: Type => { trace ! ("creating typeid alloc ID") ; let ty = Decodable :: decode (decoder) ; trace ! ("decoded typid: {ty:?}") ; decoder . interner () . reserve_and_set_type_id_alloc (ty) } AllocDiscriminant :: Static => { trace ! ("creating extern static alloc ID") ; let did = < DefId as Decodable < D > > :: decode (decoder) ; trace ! ("decoded static def-ID: {:?}" , did) ; decoder . interner () . reserve_and_set_static_alloc (did) } }) ; * entry = State :: Done (alloc_id) ; alloc_id } }}}
mkitem!{mkenum!{# [doc = " An allocation in the global (tcx-managed) memory can be either a function pointer,"] # [doc = " a static, or a \"real\" allocation with some data in it."] # [derive (Debug , Clone , Eq , PartialEq , Hash , TyDecodable , TyEncodable , HashStable)] pub enum GlobalAlloc < 'tcx > { # [doc = " The alloc ID is used as a function pointer."] Function { instance : Instance < 'tcx > } , # [doc = " This alloc ID points to a symbolic (not-reified) vtable."] # [doc = " We remember the full dyn type, not just the principal trait, so that"] # [doc = " const-eval and Miri can detect UB due to invalid transmutes of"] # [doc = " `dyn Trait` types."] VTable (Ty < 'tcx > , & 'tcx ty :: List < ty :: PolyExistentialPredicate < 'tcx > >) , # [doc = " The alloc ID points to a \"lazy\" static variable that did not get computed (yet)."] # [doc = " This is also used to break the cycle in recursive statics."] Static (DefId) , # [doc = " The alloc ID points to memory."] Memory (ConstAllocation < 'tcx >) , # [doc = " The first pointer-sized segment of a type id. On 64 bit systems, the 128 bit type id"] # [doc = " is split into two segments, on 32 bit systems there are 4 segments, and so on."] TypeId { ty : Ty < 'tcx > } , }}}
mkitem!{mkimpl!{impl < 'tcx > GlobalAlloc < 'tcx > { # [doc = " Panics if the `GlobalAlloc` does not refer to an `GlobalAlloc::Memory`"] # [track_caller] # [inline] pub fn unwrap_memory (& self) -> ConstAllocation < 'tcx > { match * self { GlobalAlloc :: Memory (mem) => mem , _ => bug ! ("expected memory, got {:?}" , self) , } } # [doc = " Panics if the `GlobalAlloc` is not `GlobalAlloc::Function`"] # [track_caller] # [inline] pub fn unwrap_fn (& self) -> Instance < 'tcx > { match * self { GlobalAlloc :: Function { instance , .. } => instance , _ => bug ! ("expected function, got {:?}" , self) , } } # [doc = " Panics if the `GlobalAlloc` is not `GlobalAlloc::VTable`"] # [track_caller] # [inline] pub fn unwrap_vtable (& self) -> (Ty < 'tcx > , Option < ty :: PolyExistentialTraitRef < 'tcx > >) { match * self { GlobalAlloc :: VTable (ty , dyn_ty) => (ty , dyn_ty . principal ()) , _ => bug ! ("expected vtable, got {:?}" , self) , } } # [doc = " The address space that this `GlobalAlloc` should be placed in."] # [inline] pub fn address_space (& self , cx : & impl HasDataLayout) -> AddressSpace { match self { GlobalAlloc :: Function { .. } => cx . data_layout () . instruction_address_space , GlobalAlloc :: TypeId { .. } | GlobalAlloc :: Static (..) | GlobalAlloc :: Memory (..) | GlobalAlloc :: VTable (..) => AddressSpace :: ZERO , } } pub fn mutability (& self , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx >) -> Mutability { match self { GlobalAlloc :: Static (did) => { let DefKind :: Static { safety : _ , mutability , nested } = tcx . def_kind (did) else { bug ! () } ; if nested { if cfg ! (debug_assertions) { let alloc = tcx . eval_static_initializer (did) . unwrap () ; assert_eq ! (alloc . 0 . mutability , mutability) ; } mutability } else { let mutability = match mutability { Mutability :: Not if ! tcx . type_of (did) . no_bound_vars () . expect ("statics should not have generic parameters") . is_freeze (tcx , typing_env) => { Mutability :: Mut } _ => mutability , } ; mutability } } GlobalAlloc :: Memory (alloc) => alloc . inner () . mutability , GlobalAlloc :: TypeId { .. } | GlobalAlloc :: Function { .. } | GlobalAlloc :: VTable (..) => { Mutability :: Not } } } pub fn size_and_align (& self , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> (Size , Align) { match self { GlobalAlloc :: Static (def_id) => { let DefKind :: Static { nested , .. } = tcx . def_kind (def_id) else { bug ! ("GlobalAlloc::Static is not a static") } ; if nested { let alloc = tcx . eval_static_initializer (def_id) . unwrap () ; (alloc . 0 . size () , alloc . 0 . align) } else { let ty = tcx . type_of (def_id) . no_bound_vars () . expect ("statics should not have generic parameters") ; let layout = tcx . layout_of (typing_env . as_query_input (ty)) . unwrap () ; assert ! (layout . is_sized ()) ; let align = match tcx . codegen_fn_attrs (def_id) . alignment { Some (align_from_attribute) => { Ord :: max (align_from_attribute , layout . align . abi) } None => layout . align . abi , } ; (layout . size , align) } } GlobalAlloc :: Memory (alloc) => { let alloc = alloc . inner () ; (alloc . size () , alloc . align) } GlobalAlloc :: Function { .. } => (Size :: ZERO , Align :: ONE) , GlobalAlloc :: VTable (..) => { (Size :: ZERO , tcx . data_layout . pointer_align () . abi) } GlobalAlloc :: TypeId { .. } => (Size :: ZERO , Align :: ONE) , } } }}}
mkitem!{pub const CTFE_ALLOC_SALT : usize = 0 ;}
mkitem!{mkstruct!{pub (crate) struct AllocMap < 'tcx > { # [doc = " Maps `AllocId`s to their corresponding allocations."] to_alloc : ShardedHashMap < AllocId , GlobalAlloc < 'tcx > > , # [doc = " Used to deduplicate global allocations: functions, vtables, string literals, ..."] # [doc = ""] # [doc = " The `usize` is a \"salt\" used by Miri to make deduplication imperfect, thus better emulating"] # [doc = " the actual guarantees."] dedup : Lock < FxHashMap < (GlobalAlloc < 'tcx > , usize) , AllocId > > , # [doc = " The `AllocId` to assign to the next requested ID."] # [doc = " Always incremented; never gets smaller."] next_id : AtomicU64 , }}}
mkitem!{mkimpl!{impl < 'tcx > AllocMap < 'tcx > { pub (crate) fn new () -> Self { AllocMap { to_alloc : Default :: default () , dedup : Default :: default () , next_id : AtomicU64 :: new (1) , } } fn reserve (& self) -> AllocId { let next_id = self . next_id . fetch_add (1 , std :: sync :: atomic :: Ordering :: Relaxed) ; AllocId (NonZero :: new (next_id) . unwrap ()) } }}}
mkitem!{mkimpl!{impl < 'tcx > TyCtxt < 'tcx > { # [doc = " Obtains a new allocation ID that can be referenced but does not"] # [doc = " yet have an allocation backing it."] # [doc = ""] # [doc = " Make sure to call `set_alloc_id_memory` or `set_alloc_id_same_memory` before returning such"] # [doc = " an `AllocId` from a query."] pub fn reserve_alloc_id (self) -> AllocId { self . alloc_map . reserve () } # [doc = " Reserves a new ID *if* this allocation has not been dedup-reserved before."] # [doc = " Should not be used for mutable memory."] fn reserve_and_set_dedup (self , alloc : GlobalAlloc < 'tcx > , salt : usize) -> AllocId { if let GlobalAlloc :: Memory (mem) = alloc { if mem . inner () . mutability . is_mut () { bug ! ("trying to dedup-reserve mutable memory") ; } } let alloc_salt = (alloc , salt) ; let mut dedup = self . alloc_map . dedup . lock () ; if let Some (& alloc_id) = dedup . get (& alloc_salt) { return alloc_id ; } let id = self . alloc_map . reserve () ; debug ! ("creating alloc {:?} with id {id:?}" , alloc_salt . 0) ; let had_previous = self . alloc_map . to_alloc . insert (id , alloc_salt . 0 . clone ()) . is_some () ; assert ! (! had_previous) ; dedup . insert (alloc_salt , id) ; id } # [doc = " Generates an `AllocId` for a memory allocation. If the exact same memory has been"] # [doc = " allocated before, this will return the same `AllocId`."] pub fn reserve_and_set_memory_dedup (self , mem : ConstAllocation < 'tcx > , salt : usize) -> AllocId { self . reserve_and_set_dedup (GlobalAlloc :: Memory (mem) , salt) } # [doc = " Generates an `AllocId` for a static or return a cached one in case this function has been"] # [doc = " called on the same static before."] pub fn reserve_and_set_static_alloc (self , static_id : DefId) -> AllocId { let salt = 0 ; self . reserve_and_set_dedup (GlobalAlloc :: Static (static_id) , salt) } # [doc = " Generates an `AllocId` for a function. Will get deduplicated."] pub fn reserve_and_set_fn_alloc (self , instance : Instance < 'tcx > , salt : usize) -> AllocId { self . reserve_and_set_dedup (GlobalAlloc :: Function { instance } , salt) } # [doc = " Generates an `AllocId` for a (symbolic, not-reified) vtable. Will get deduplicated."] pub fn reserve_and_set_vtable_alloc (self , ty : Ty < 'tcx > , dyn_ty : & 'tcx ty :: List < ty :: PolyExistentialPredicate < 'tcx > > , salt : usize ,) -> AllocId { self . reserve_and_set_dedup (GlobalAlloc :: VTable (ty , dyn_ty) , salt) } # [doc = " Generates an [AllocId] for a [core::any::TypeId]. Will get deduplicated."] pub fn reserve_and_set_type_id_alloc (self , ty : Ty < 'tcx >) -> AllocId { self . reserve_and_set_dedup (GlobalAlloc :: TypeId { ty } , 0) } # [doc = " Interns the `Allocation` and return a new `AllocId`, even if there's already an identical"] # [doc = " `Allocation` with a different `AllocId`."] # [doc = " Statics with identical content will still point to the same `Allocation`, i.e.,"] # [doc = " their data will be deduplicated through `Allocation` interning -- but they"] # [doc = " are different places in memory and as such need different IDs."] pub fn reserve_and_set_memory_alloc (self , mem : ConstAllocation < 'tcx >) -> AllocId { let id = self . reserve_alloc_id () ; self . set_alloc_id_memory (id , mem) ; id } # [doc = " Returns `None` in case the `AllocId` is dangling. An `InterpretCx` can still have a"] # [doc = " local `Allocation` for that `AllocId`, but having such an `AllocId` in a constant is"] # [doc = " illegal and will likely ICE."] # [doc = " This function exists to allow const eval to detect the difference between evaluation-"] # [doc = " local dangling pointers and allocations in constants/statics."] # [inline] pub fn try_get_global_alloc (self , id : AllocId) -> Option < GlobalAlloc < 'tcx > > { self . alloc_map . to_alloc . get (& id) } # [inline] # [track_caller] # [doc = " Panics in case the `AllocId` is dangling. Since that is impossible for `AllocId`s in"] # [doc = " constants (as all constants must pass interning and validation that check for dangling"] # [doc = " ids), this function is frequently used throughout rustc, but should not be used within"] # [doc = " the interpreter."] pub fn global_alloc (self , id : AllocId) -> GlobalAlloc < 'tcx > { match self . try_get_global_alloc (id) { Some (alloc) => alloc , None => bug ! ("could not find allocation for {id:?}") , } } # [doc = " Freezes an `AllocId` created with `reserve` by pointing it at an `Allocation`. Trying to"] # [doc = " call this function twice, even with the same `Allocation` will ICE the compiler."] pub fn set_alloc_id_memory (self , id : AllocId , mem : ConstAllocation < 'tcx >) { if let Some (old) = self . alloc_map . to_alloc . insert (id , GlobalAlloc :: Memory (mem)) { bug ! ("tried to set allocation ID {id:?}, but it was already existing as {old:#?}") ; } } # [doc = " Freezes an `AllocId` created with `reserve` by pointing it at a static item. Trying to"] # [doc = " call this function twice, even with the same `DefId` will ICE the compiler."] pub fn set_nested_alloc_id_static (self , id : AllocId , def_id : LocalDefId) { if let Some (old) = self . alloc_map . to_alloc . insert (id , GlobalAlloc :: Static (def_id . to_def_id ())) { bug ! ("tried to set allocation ID {id:?}, but it was already existing as {old:#?}") ; } } }}}

macro_rules! write_target_uint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function write_target_uint in module {}", module_path!());
    };
}

mkfn!{
    write_target_uint_introspect!();
    # [inline] pub fn write_target_uint (endianness : Endian , mut target : & mut [u8] , data : u128 ,) -> Result < () , io :: Error > { match endianness { Endian :: Little => target . write (& data . to_le_bytes ()) ? , Endian :: Big => target . write (& data . to_be_bytes () [16 - target . len () ..]) ? , } ; debug_assert ! (target . len () == 0) ; Ok (()) }
}

macro_rules! read_target_uint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read_target_uint in module {}", module_path!());
    };
}

mkfn!{
    read_target_uint_introspect!();
    # [inline] pub fn read_target_uint (endianness : Endian , mut source : & [u8]) -> Result < u128 , io :: Error > { let mut buf = [0u8 ; size_of :: < u128 > ()] ; let uint = match endianness { Endian :: Little => { source . read_exact (& mut buf [.. source . len ()]) ? ; Ok (u128 :: from_le_bytes (buf)) } Endian :: Big => { source . read_exact (& mut buf [16 - source . len () ..]) ? ; Ok (u128 :: from_be_bytes (buf)) } } ; debug_assert ! (source . len () == 0) ; uint }
}