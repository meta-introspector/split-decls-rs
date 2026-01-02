mkuse!{pub (crate) use rustc_data_structures :: fx :: { FxIndexMap as Map , FxIndexSet as Set } ;}
mkmod!{layout, { 
                getname!(layout);
                getsrc!(layout);
                getpath!(layout);
                get_deps!(layout);
                get_crates!(layout);
                mkinclude!(layout);
                 
            }}
mkmod!{maybe_transmutable, { 
                getname!(maybe_transmutable);
                getsrc!(maybe_transmutable);
                getpath!(maybe_transmutable);
                get_deps!(maybe_transmutable);
                get_crates!(maybe_transmutable);
                mkinclude!(maybe_transmutable);
                 
            }}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , Default)] pub struct Assume { pub alignment : bool , pub lifetimes : bool , pub safety : bool , pub validity : bool , }}}
mkitem!{mkenum!{# [doc = " Either transmutation is allowed, we have an error, or we have an optional"] # [doc = " Condition that must hold."] # [derive (Debug , Hash , Eq , PartialEq , Clone)] pub enum Answer < R , T > { Yes , No (Reason < T >) , If (Condition < R , T >) , }}}
mkitem!{mkenum!{# [doc = " A condition which must hold for safe transmutation to be possible."] # [derive (Debug , Hash , Eq , PartialEq , Clone)] pub enum Condition < R , T > { # [doc = " `Src` is transmutable into `Dst`, if `src` is transmutable into `dst`."] Transmutable { src : T , dst : T } , # [doc = " The region `long` must outlive `short`."] Outlives { long : R , short : R } , # [doc = " The `ty` is immutable."] Immutable { ty : T } , # [doc = " `Src` is transmutable into `Dst`, if all of the enclosed requirements are met."] IfAll (Vec < Condition < R , T > >) , # [doc = " `Src` is transmutable into `Dst` if any of the enclosed requirements are met."] IfAny (Vec < Condition < R , T > >) , }}}
mkitem!{mkenum!{# [doc = " Answers \"why wasn't the source type transmutable into the destination type?\""] # [derive (Debug , Hash , Eq , PartialEq , PartialOrd , Ord , Clone)] pub enum Reason < T > { # [doc = " The layout of the source type is not yet supported."] SrcIsNotYetSupported , # [doc = " The layout of the destination type is not yet supported."] DstIsNotYetSupported , # [doc = " The layout of the destination type is bit-incompatible with the source type."] DstIsBitIncompatible , # [doc = " The destination type is uninhabited."] DstUninhabited , # [doc = " The destination type may carry safety invariants."] DstMayHaveSafetyInvariants , # [doc = " `Dst` is larger than `Src`, and the excess bytes were not exclusively uninitialized."] DstIsTooBig , # [doc = " `Dst` is larger `Src`."] DstRefIsTooBig { # [doc = " The referent of the source type."] src : T , # [doc = " The size of the source type's referent."] src_size : usize , # [doc = " The too-large referent of the destination type."] dst : T , # [doc = " The size of the destination type's referent."] dst_size : usize , } , # [doc = " Src should have a stricter alignment than Dst, but it does not."] DstHasStricterAlignment { src_min_align : usize , dst_min_align : usize } , # [doc = " Can't go from shared pointer to unique pointer"] DstIsMoreUnique , # [doc = " Encountered a type error"] TypeError , # [doc = " The layout of src is unknown"] SrcLayoutUnknown , # [doc = " The layout of dst is unknown"] DstLayoutUnknown , # [doc = " The size of src is overflow"] SrcSizeOverflow , # [doc = " The size of dst is overflow"] DstSizeOverflow , }}}
mkmod!{rustc, { 
                getname!(rustc);
                getsrc!(rustc);
                getpath!(rustc);
                get_deps!(rustc);
                get_crates!(rustc);
                mkinclude!(rustc);
                mkuse!{use rustc_hir :: lang_items :: LangItem ;}
mkuse!{use rustc_middle :: ty :: { Const , Region , Ty , TyCtxt } ;}
mkuse!{use super :: * ;}
mkitem!{mkstruct!{# [doc = " The source and destination types of a transmutation."] # [derive (Debug , Clone , Copy)] pub struct Types < 'tcx > { # [doc = " The source type."] pub src : Ty < 'tcx > , # [doc = " The destination type."] pub dst : Ty < 'tcx > , }}}
mkitem!{mkstruct!{pub struct TransmuteTypeEnv < 'tcx > { tcx : TyCtxt < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > TransmuteTypeEnv < 'tcx > { pub fn new (tcx : TyCtxt < 'tcx >) -> Self { Self { tcx } } pub fn is_transmutable (& mut self , types : Types < 'tcx > , assume : crate :: Assume ,) -> crate :: Answer < Region < 'tcx > , Ty < 'tcx > > { crate :: maybe_transmutable :: MaybeTransmutableQuery :: new (types . src , types . dst , assume , self . tcx ,) . answer () } }}}
mkitem!{mkimpl!{impl Assume { # [doc = " Constructs an `Assume` from a given const-`Assume`."] pub fn from_const < 'tcx > (tcx : TyCtxt < 'tcx > , ct : Const < 'tcx >) -> Option < Self > { use rustc_middle :: ty :: ScalarInt ; use rustc_span :: sym ; let Some (cv) = ct . try_to_value () else { return None ; } ; let adt_def = cv . ty . ty_adt_def () ? ; if ! tcx . is_lang_item (adt_def . did () , LangItem :: TransmuteOpts) { tcx . dcx () . delayed_bug (format ! ("The given `const` was not marked with the `{}` lang item." , LangItem :: TransmuteOpts . name ())) ; return Some (Self { alignment : true , lifetimes : true , safety : true , validity : true , }) ; } let variant = adt_def . non_enum_variant () ; let fields = cv . valtree . unwrap_branch () ; let get_field = | name | { let (field_idx , _) = variant . fields . iter () . enumerate () . find (| (_ , field_def) | name == field_def . name) . unwrap_or_else (| | panic ! ("There were no fields named `{name}`.")) ; fields [field_idx] . unwrap_leaf () == ScalarInt :: TRUE } ; Some (Self { alignment : get_field (sym :: alignment) , lifetimes : get_field (sym :: lifetimes) , safety : get_field (sym :: safety) , validity : get_field (sym :: validity) , }) } }}} 
            }}
mkuse!{# [cfg (feature = "rustc")] pub use rustc :: * ;}